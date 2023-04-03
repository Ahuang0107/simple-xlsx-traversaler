#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Row {
    #[serde(rename = "@r")]
    pub(crate) num: Option<String>,
    #[serde(rename = "@spans")]
    pub(crate) spans: Option<String>,
    pub(crate) c: Vec<C>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct C {
    /// such as A1, B1
    #[serde(rename = "@r")]
    pub(crate) num: Option<String>,
    /// if the value is shared string
    #[serde(rename = "@t")]
    pub(crate) t: Option<String>,
    pub(crate) v: Option<String>,
}
