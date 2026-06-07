pub mod query_parser;

use crate::scryfall::ScryfallStorage;
use polars::error::PolarsResult;
use std::ops::Deref;

impl ScryfallStorage {
    pub fn search(
        &self,
        _query: String,
        _cols: impl Deref<Target = [impl Deref<Target = str>]>,
    ) -> PolarsResult<()> {
        // let lf = self.lf.as_ref().unwrap().clone();

        // let select = cols.iter()
        //     .map(|field| col(field.deref()))
        //     .collect::<Vec<Expr>>();
        //
        // let df = lf.select(select)
        //     .filter()
        //     .collect()?;

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::scryfall::ScryfallStorage;
    use clients::scryfall::ScryfallClient;
    use polars::prelude::{col, lit};
    use std::time::Instant;

    #[test]
    fn test_search_by_scryfall_id() {
        let storage = ScryfallStorage::new(ScryfallClient::new());

        let start = Instant::now();
        let lf_guard = storage.cards_lf.lock().unwrap();
        let lf = lf_guard.as_ref().unwrap().clone();
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
