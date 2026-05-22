use crate::generate_record_builder_and_dt;
use serde::{Deserialize, Serialize};
use url::Url;

generate_record_builder_and_dt! {
    #[derive(Debug, Serialize, Deserialize)]
    PurchaseUris {
        tcgplayer: Option<Url>,
        cardmarket: Option<Url>,
        cardhoarder: Option<Url>,
    }
}
