pub mod query_parser;
pub mod polars_mapping;

use crate::scryfall::ScryfallStorage;
use polars::prelude::DataFrame;
use schemas::oshibana::SearchColumn;
use std::ops::Deref;
use thiserror::Error;
use crate::scryfall::search::query_parser::Query;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("polars error: {0}")]
    PolarsError(#[from] polars::error::PolarsError),
    #[error("query parsing error: {0}")]
    QueryParseError(#[from] pest::error::Error<query_parser::Rule>),
    #[error("query is empty")]
    EmptyQuery
}

impl ScryfallStorage {
    pub fn search(
        &self,
        query: &str,
        cols: impl Deref<Target = [SearchColumn]>,
    ) -> Result<DataFrame, SearchError> {
        let query = Query::parse(query)?;

        let Some(query) = query else {
            return Err(SearchError::EmptyQuery);
        };

        let filtered = polars_mapping::apply_filters(query, self);
        let results_lf = polars_mapping::apply_select(filtered, cols);
        let result = results_lf.collect()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::scryfall::ScryfallStorage;
    use clients::scryfall::ScryfallClient;
    use polars::prelude::{col, lit, IntoLazy};
    use std::time::Instant;

    #[test]
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
