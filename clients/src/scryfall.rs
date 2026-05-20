//! Scryfall client

use futures::{Stream, StreamExt, stream};
use governor::clock::{Clock, DefaultClock};
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use reqwest::header::HeaderMap;
use reqwest::{Client, IntoUrl, Method, Url};
use schemas::scryfall::bulk_data::BulkData;
use schemas::scryfall::card::ScryfallCard;
use schemas::scryfall::lists::ScryfallList;
use schemas::scryfall::set::ScryfallSet;
use serde::de::DeserializeOwned;
use std::collections::VecDeque;
use std::io;
use std::io::ErrorKind;
use std::num::NonZeroU32;
use std::sync::Arc;
use thiserror::Error;
use tokio_util::io::{StreamReader, SyncIoBridge};

#[derive(Clone, Debug)]
pub struct ScryfallClient {
    pub client: Client,
    rl: Arc<DefaultDirectRateLimiter>,
}

#[derive(Error, Debug)]
pub enum ScryfallClientError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl ScryfallClient {
    const ROOT_URL: &'static str = "https://api.scryfall.com";

    pub fn new() -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            "User-Agent",
            format!("oshibana/{}", env!("CARGO_PKG_VERSION"))
                .parse()
                .unwrap(),
        );
        headers.insert("Accept", "application/json".parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        ScryfallClient {
            client,
            rl: Arc::new(RateLimiter::direct(Quota::per_second(
                const { NonZeroU32::new(10).unwrap() },
            ))),
        }
    }

    async fn call_raw<T: DeserializeOwned>(
        &self,
        method: Method,
        url: impl IntoUrl,
    ) -> reqwest::Result<T> {
        if let Err(not_until) = self.rl.check() {
            let now = DefaultClock::default().now();
            log::warn!(
                "Hit Scryfall API ratelimit, waiting {:?}",
                not_until.wait_time_from(now)
            );
            self.rl.until_ready().await;
        }

        self.client.request(method, url).send().await?.json().await
    }

    async fn call<T: DeserializeOwned>(
        &self,
        method: Method,
        route: impl AsRef<str>,
    ) -> reqwest::Result<T> {
        self.call_raw(method, format!("{}/{}", Self::ROOT_URL, route.as_ref()))
            .await
    }

    pub async fn bulk_data(&self) -> reqwest::Result<BulkData> {
        self.call(Method::GET, "bulk-data").await
    }

    pub async fn all_sets(&self) -> reqwest::Result<ScryfallList<ScryfallSet>> {
        self.call(Method::GET, "sets").await
    }

    /// Get a [Stream] over all scryfall sets printed. Discards any warnings attached to the
    /// returned [ScryfallList].
    pub async fn all_sets_stream(
        self,
    ) -> reqwest::Result<impl Stream<Item = reqwest::Result<ScryfallSet>>> {
        let init_list_object = self.all_sets().await?;

        Ok(stream::try_unfold(
            init_list_object,
            move |mut list_object| {
                let client = self.clone();
                async move {
                    // as long as we can pull from the current list object, do that.
                    if list_object.data.len() > 0 {
                        let next = list_object.data.remove(0);
                        Ok(Some((next, list_object)))
                    } else if let Some(next_page) = list_object.next_page {
                        let mut new_list_object: ScryfallList<ScryfallSet> =
                            client.call_raw(Method::GET, next_page).await?;

                        let next = new_list_object.data.remove(0);
                        Ok(Some((next, new_list_object)))
                    } else {
                        Ok(None)
                    }
                }
            },
        ))
    }
}
