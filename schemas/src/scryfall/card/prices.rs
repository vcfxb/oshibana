use crate::generate_record_builder_and_dt;
use serde::{Deserialize, Serialize};

generate_record_builder_and_dt! {
    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    Prices {
        usd: Option<f32>,
        usd_foil: Option<f32>,
        usd_etched: Option<f32>,
        eur: Option<f32>,
        eur_foil: Option<f32>,
        tix: Option<f32>,
    }
}
