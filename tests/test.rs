use simple_xlsx_traversaler::{get_sheet_names, traversal};

#[test]
fn read_all_sheet_names() {
    let file = std::fs::File::open("tests/test.xlsx").unwrap();
    let sheet_names = get_sheet_names(&file).unwrap();
    assert_eq!(sheet_names, vec!["Sheet1", "测试sheet2"]);
}

#[test]
fn read_top_100_rows() {
    let file = std::fs::File::open("tests/test.xlsx").unwrap();
    let data = traversal(&file, "测试sheet2", 100).unwrap();
    assert_eq!(data.len(), 100);
}
