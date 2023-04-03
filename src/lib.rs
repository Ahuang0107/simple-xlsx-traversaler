mod serde_struct;
mod utils;

pub fn traversal(
    file: std::fs::File,
    row_limit: usize,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let shared_strings: serde_struct::SharedString = {
        let reader = std::io::BufReader::new(&file);
        let mut archive = zip::ZipArchive::new(reader)?;
        let shared_strings_zip = archive.by_name("xl/sharedStrings.xml")?;
        let shared_strings_reader = std::io::BufReader::new(shared_strings_zip);
        quick_xml::de::from_reader(shared_strings_reader)?
    };

    let reader = std::io::BufReader::new(&file);
    let mut archive = zip::ZipArchive::new(reader)?;
    let mut sheet_zip = archive.by_name("xl/worksheets/sheet1.xml")?;
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
            if col.t.is_some() && col.v.is_some() {
                let value = col.v.unwrap().parse::<usize>()?;
                let value = &shared_strings.si[value];
                row_data.push(value.t.clone());
            } else if col.v.is_some() {
                row_data.push(col.v.unwrap().clone());
            }
        }
        data.push(row_data);
    }

    Ok(data)
}
