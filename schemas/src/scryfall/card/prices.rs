use crate::generate_record_builder_and_dt;
use serde::{Deserialize, Serialize};

generate_record_builder_and_dt! {
    #[derive(Debug, Serialize, Deserialize)]
    Prices {
        usd: Option<String>,
        usd_foil: Option<String>,
        usd_etched: Option<String>,
        eur: Option<String>,
        eur_foil: Option<String>,
        tix: Option<String>,
    }
}
