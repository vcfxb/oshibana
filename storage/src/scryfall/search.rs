pub mod polars_mapping;
pub mod query_parser;

use crate::scryfall::ScryfallStorage;
use crate::scryfall::search::query_parser::Parser;
use polars::prelude::DataFrame;
use schemas::oshibana::SearchColumn;
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
    pub fn search(
        &self,
        query: &str,
        cols: impl Deref<Target = [SearchColumn]>,
    ) -> Result<DataFrame, SearchError> {
        let mut parser = Parser::new(query);
        let parsed_query = parser.parse_query();
        
        if parsed_query.intersections.is_empty() {
            return Err(SearchError::EmptyQuery);
        }

        let filtered = polars_mapping::apply_filters(&parsed_query, self);
        let results_lf = polars_mapping::apply_select(filtered, cols);
        let result = results_lf.collect()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::scryfall::ScryfallStorage;
    use clients::scryfall::ScryfallClient;
    use polars::prelude::{IntoLazy, col, lit};
    use std::time::Instant;

    #[test]
    #[ignore = "this tests only works when run locally with a valid scryfall sync file"]
    fn test_search_by_scryfall_id() {
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
