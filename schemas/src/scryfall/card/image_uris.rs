use crate::generate_record_builder_and_dt;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

generate_record_builder_and_dt! {
    #[derive(Debug, Serialize, Deserialize)]
    ImageUris {
        small: Option<Url>,
        normal: Option<Url>,
        large: Option<Url>,
        png: Option<Url>,
        art_crop: Option<Url>,
        border_crop: Option<Url>
    }
}
