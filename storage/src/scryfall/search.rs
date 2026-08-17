pub mod polars_mapping;
pub mod query_parser;

use crate::scryfall::ScryfallStorage;
use crate::scryfall::search::query_parser::{Diagnostic, Parser};
use polars::prelude::{DataFrame, IntoLazy};
use schemas::oshibana::{SearchViewColumn, UniqueBy};
use std::ops::Deref;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("polars error: {0}")]
    PolarsError(#[from] polars::error::PolarsError),
    #[error("query is empty")]
    EmptyQuery,
}

impl ScryfallStorage {
    pub fn search<'i>(
        &self,
        query: &'i str,
        cols: impl Deref<Target = [SearchViewColumn]>,
        unique_by: UniqueBy,
    ) -> (Result<DataFrame, SearchError>, Vec<Diagnostic<'i>>) {
        let mut parser = Parser::new(query);
        let parsed_query = parser.parse_query();
        
        if parsed_query.intersections.is_empty() {
            return (Err(SearchError::EmptyQuery), parser.diagnostics);
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
        let grouped = polars_mapping::apply_grouping(filtered, unique_by);
        let results_lf = polars_mapping::apply_select(grouped, cols);
        (results_lf.collect().map_err(Into::into), parser.diagnostics)
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
        use polars::prelude::{IntoLazy, col, lit};
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
