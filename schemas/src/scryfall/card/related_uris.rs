use crate::generate_record_builder_and_dt;
use serde::{Deserialize, Serialize};
use url::Url;

generate_record_builder_and_dt! {
    #[derive(Debug, Serialize, Deserialize)]
    RelatedUris {
        gatherer: Option<Url>,
        tcgplayer_infinite_articles: Option<Url>,
        tcgplayer_infinite_decks: Option<Url>,
        edhrec: Option<Url>,
    }
}
