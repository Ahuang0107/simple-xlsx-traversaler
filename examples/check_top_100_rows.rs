use simple_xlsx_traversaler::{get_sheet_names, traversal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("examples/example.xlsx")?;
    let sheet_names = get_sheet_names(&file)?;
    for sheet in sheet_names {
        let data = traversal(&file, sheet.as_str(), 100)?;
        println!("{sheet} read {} rows", data.len());
    }
    Ok(())
}
