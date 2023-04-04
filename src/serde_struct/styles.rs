#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct StyleSheet {
    #[serde(rename = "cellXfs")]
    pub(crate) cell_xfs: CellXfs,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct CellXfs {
    pub(crate) xf: Vec<Xf>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Xf {
    #[serde(rename = "@numFmtId")]
    pub(crate) num_fmt_id: String,
}
