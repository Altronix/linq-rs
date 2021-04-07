use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct About {
    pub siteId: String,
    pub prjVersion: String,
    pub prjVersionRc: String,
    pub atxVersion: String,
    pub atxVersionRc: String,
    pub sid: String,
    pub mac: String,
    pub product: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AboutResponse {
    pub about: About,
}
