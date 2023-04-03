#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Relationships {
    pub(crate) relationship: Vec<Relationship>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Relationship {
    #[serde(rename = "@Id")]
    pub(crate) id: String,
    #[serde(rename = "@Target")]
    pub(crate) target: String,
}
