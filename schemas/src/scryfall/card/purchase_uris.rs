use url::Url;
use crate::generate_record_builder_and_dt;
use serde::{Serialize, Deserialize};

generate_record_builder_and_dt! {
    #[derive(Debug, Serialize, Deserialize)]
    PurchaseUris {
        tcgplayer: Option<Url>,
        cardmarket: Option<Url>,
        cardhoarder: Option<Url>,
    }
}
