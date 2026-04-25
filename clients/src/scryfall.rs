//! Scryfall client

use schemas::scryfall::bulk_data::BulkData;
use governor::clock::{Clock, DefaultClock};
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use std::num::NonZeroU32;
use std::sync::Arc;
use thiserror::Error;

pub struct ScryfallClient {
    client: Client,
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
            format!("moxfield-cli/{}", env!("CARGO_PKG_VERSION"))
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

    async fn call<T: DeserializeOwned>(
        &self,
        method: Method,
        route: impl AsRef<str>,
    ) -> reqwest::Result<T> {
        if let Err(not_until) = self.rl.check() {
            let now = DefaultClock::default().now();
            log::warn!(
                "Hit Scryfall API ratelimit, waiting {:?}",
                not_until.wait_time_from(now)
            );
            self.rl.until_ready().await;
        }

        self.client
            .request(method, format!("{}/{}", Self::ROOT_URL, route.as_ref()))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn bulk_data(&self) -> reqwest::Result<BulkData> {
        self.call(Method::GET, "bulk-data").await
    }
}
