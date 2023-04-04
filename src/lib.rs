mod serde_struct;
mod utils;

pub struct Xlsx {
    file: std::fs::File,
    workbook_info: serde_struct::Workbook,
    relationships_info: serde_struct::Relationships,
    shared_strings_info: serde_struct::SharedString,
    styles_info: serde_struct::StyleSheet,
}

impl Xlsx {
    pub fn new(file: std::fs::File) -> Result<Self, Box<dyn std::error::Error>> {
        let workbook_info = utils::serde_from(&file, "xl/workbook.xml")?;
        let relationships_info = utils::serde_from(&file, "xl/_rels/workbook.xml.rels")?;
        let styles_info = utils::serde_from(&file, "xl/styles.xml")?;
        let shared_strings_info = utils::serde_from(&file, "xl/sharedStrings.xml")?;
        Ok(Self {
            file,
            workbook_info,
            relationships_info,
            shared_strings_info,
            styles_info,
        })
    }

    pub fn sheet_list(&self) -> Vec<String> {
        self.workbook_info
            .sheets
            .sheet
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }

    pub fn traversal(
        &self,
        sheet: &str,
        row_limit: usize,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let sheet = self
            .workbook_info
            .sheets
            .sheet
            .iter()
            .find(|s| s.name == sheet)
            .unwrap();
        let rel = self
            .relationships_info
            .relationship
            .iter()
            .find(|rel| rel.id == sheet.id)
            .unwrap();

        let reader = std::io::BufReader::new(&self.file);
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut sheet_zip = archive.by_name(&format!("xl/{}", rel.target))?;
        const CHUNK_LEN: usize = 1000;
        let mut buffer = [0u8; CHUNK_LEN];
        let mut in_sheet_data = false;
        let mut string = String::new();
        loop {
            use std::io::Read;

            let read_count = sheet_zip.read(&mut buffer)?;
            let cur_string = String::from_utf8_lossy(&buffer[..read_count]);
            if cur_string.contains(r"<sheetData>") {
                let pair: Vec<_> = cur_string.split(r"<sheetData>").collect();
                assert_eq!(pair.len(), 2);
                string.extend(pair[1].chars().into_iter());
                in_sheet_data = true;
            } else if cur_string.contains(r"</sheetData>") {
                let pair: Vec<_> = cur_string.split(r"</sheetData>").collect();
                assert_eq!(pair.len(), 2);
                string.extend(pair[0].replace("</sheetData>", "").chars().into_iter());
                in_sheet_data = false;
            } else if in_sheet_data {
                string.extend(cur_string.chars().into_iter());
            }
            if read_count == 0 {
                break;
            }
            let matches = string.match_indices("</row>").collect::<Vec<_>>();
            if matches.len() >= row_limit {
                let (index, _) = matches[row_limit - 1];
                string = string.split_at(index + "</row>".len()).0.to_string();
                break;
            }
        }

        let rows: Vec<serde_struct::Row> = quick_xml::de::from_str(string.as_str())?;
        let mut data: Vec<Vec<String>> = vec![];
        for row in rows {
            let mut row_data: Vec<String> = vec![];
            for col in row.c {
                if col.v.is_some() {
                    if let Some(t) = col.t {
                        match t.as_str() {
                            "s" => {
                                // shared string
                                let value = col.v.unwrap().parse::<usize>()?;
                                let value = &self.shared_strings_info.si[value];
                                row_data.push(value.t.clone());
                            }
                            _ => {
                                row_data.push(col.v.unwrap().clone());
                            }
                        }
                    } else {
                        if let Some(s) = col.s {
                            if let Ok(s) = s.parse::<usize>() {
                                let format_id = &self.styles_info.cell_xfs.xf[s].num_fmt_id;
                                match format_id.as_str() {
                                    // mm-dd-yy
                                    "14" => {
                                        let value = col.v.unwrap().parse::<i64>()?;
                                        let days = value - 25569;
                                        let secs = days * 86400;
                                        row_data.push(
                                            chrono::NaiveDateTime::from_timestamp_opt(secs, 0)
                                                .unwrap()
                                                .format("%F")
                                                .to_string(),
                                        );
                                    }
                                    _ => {
                                        row_data.push(col.v.unwrap().clone());
                                    }
                                }
                            }
                        } else {
                            row_data.push(col.v.unwrap().clone());
                        }
                    }
                }
            }
            data.push(row_data);
        }

        Ok(data)
    }
}
