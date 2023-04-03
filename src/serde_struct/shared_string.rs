#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "sst")]
pub(crate) struct SharedString {
    pub(crate) si: Vec<Si>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Si {
    pub(crate) t: String,
}
