use simple_xlsx_traversaler::Xlsx;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xlsx = Xlsx::new(std::fs::File::open("examples/example.xlsx")?)?;
    let sheet_names = xlsx.sheet_list();
    for sheet in sheet_names {
        let data = xlsx.traversal(sheet.as_str(), 100)?;
        println!("{sheet} read {} rows", data.len());
    }
    Ok(())
}
