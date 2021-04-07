use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Update {
    #[serde(rename = "type")]
    pub kind: String,
    pub size: u32,
    pub offset: u32,
    pub payload: String,
    pub md5: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct DashboardUpdateImage {
    pub update: Vec<Update>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct DashboardUpdate {
    pub files: Vec<DashboardUpdateImage>,
}
