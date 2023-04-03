#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Workbook {
    pub(crate) sheets: Sheets,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Sheets {
    pub(crate) sheet: Vec<Sheet>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Sheet {
    #[serde(rename = "@name")]
    pub(crate) name: String,
    #[serde(rename = "@sheetId")]
    pub(crate) sheet_id: String,
    #[serde(rename = "@id")]
    pub(crate) id: String,
}
