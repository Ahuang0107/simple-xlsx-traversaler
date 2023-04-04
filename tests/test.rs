use simple_xlsx_traversaler::Xlsx;

#[test]
fn read_all_sheet_names() {
    let xlsx = Xlsx::new(std::fs::File::open("tests/test.xlsx").unwrap()).unwrap();
    assert_eq!(xlsx.sheet_list(), vec!["Sheet1", "测试sheet2"]);
}

#[test]
fn read_top_100_rows() {
    let xlsx = Xlsx::new(std::fs::File::open("tests/test.xlsx").unwrap()).unwrap();
    let data = xlsx.traversal("测试sheet2", 100).unwrap();
    assert_eq!(data.len(), 100);
}
