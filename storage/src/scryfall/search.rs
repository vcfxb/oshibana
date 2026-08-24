pub mod polars_mapping;
pub mod query_parser;

use crate::scryfall::search::query_parser::{Diagnostic, Parser};
use crate::scryfall::ScryfallStorage;
use polars::prelude::{DataFrame, IntoLazy};
use schemas::oshibana::{SearchViewColumn, UniqueBy};
use scopeguard::defer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use thiserror::Error;

pub struct SearchHandler {
    query_tx: Sender<Query>,
    pub result: Arc<Mutex<SearchResult>>,
    pub busy: Arc<AtomicBool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Query {
    pub query: Arc<String>,
    pub cols: Vec<SearchViewColumn>,
    pub unique_by: UniqueBy
}

#[derive(Debug)]
pub struct SearchResult {
    pub result: Result<DataFrame, SearchError>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("no result (search may not have run yet)")]
    NoResult,
    #[error("polars error: {0}")]
    PolarsError(#[from] polars::error::PolarsError),
    #[error("query is empty")]
    EmptyQuery,
}

impl SearchHandler {
    pub fn new(storage: Arc<ScryfallStorage>) -> Self {
        let (tx, rx) = mpsc::channel::<Query>();
        let storage_ref = Arc::clone(&storage);
        let result = Arc::new(Mutex::new(SearchResult {
            result: Err(SearchError::NoResult),
            diagnostics: vec![],
        }));

        let result_ref = Arc::clone(&result);
        let busy = Arc::new(AtomicBool::new(false));
        let busy_ref = Arc::clone(&busy);

        thread::spawn(move || {
            let mut last_query = None;
            loop {
                match rx.recv() {
                    Err(_) => break,
                    Ok(query) => {
                        busy_ref.store(true, Ordering::Relaxed);
                        defer! {
                            busy_ref.store(false, Ordering::Relaxed);
                        }

                        if Some(&query) != last_query.as_ref() {
                            last_query = Some(query.clone());
                            let result = storage_ref.search(query);
                            let mut result_guard = result_ref.lock().unwrap();
                            *result_guard = result;
                        }
                    }
                }
            }
        });

        SearchHandler {
            query_tx: tx,
            result,
            busy,
        }
    }

    pub fn search(&self, query: Query) {
        self.query_tx.send(query).expect("failed to send query to search worker thread");
    }
}


impl ScryfallStorage {
    fn search(&self, query: Query) -> SearchResult {
        let mut parser = Parser::new(query.query.clone());
        let parsed_query = parser.parse_query();
        
        if parsed_query.intersections.is_empty() {
            return SearchResult {
                result: Err(SearchError::EmptyQuery),
                diagnostics: parser.diagnostics,
            };
        }

        // clone reference to cards dataframe, make it lazy
        let cards_lf = self
            .cards_df
            .lock()
            .unwrap()
            .as_ref()
            .expect("cards_df should not be none")
            .clone()
            .lazy();

        let processed = polars_mapping::apply_preprocessing(cards_lf);
        let filtered = polars_mapping::apply_filters(processed, &parsed_query);
        let grouped = polars_mapping::apply_grouping(filtered, query.unique_by);
        let results_lf = polars_mapping::apply_select(grouped, query.cols);

        SearchResult {
            result: results_lf.collect().map_err(Into::into),
            diagnostics: parser.diagnostics,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "scryfall-sync-tests")]
    // #[ignore = "this tests only works when run locally with a valid scryfall sync file"]
    fn test_search_by_scryfall_id() {
        use crate::scryfall::ScryfallStorage;
        use clients::scryfall::ScryfallClient;
        use polars::prelude::{col, lit, IntoLazy};
        use std::time::Instant;

        let storage = ScryfallStorage::new(ScryfallClient::new());

        let start = Instant::now();
        let lf_guard = storage.cards_df.lock().unwrap();
        // df clone here is cheap -- it should just be a few pointers/arcs.
        let lf = lf_guard.as_ref().unwrap().clone().lazy();
        let results = lf
            .filter(col("id").eq(lit("c14c07d4-6971-483a-add1-f3cdf18feae9")))
            .select([col("name")])
            .collect()
            .unwrap();

        assert_eq!(
            results,
            polars::df! { "name" => [
                "Wheel of Fortune"
            ]}
            .unwrap()
        );

        println!("{:?}", start.elapsed());
    }
}
