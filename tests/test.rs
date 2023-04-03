use simple_xlsx_traversaler::traversal;

#[test]
fn read_top_100_rows() {
    let file = std::fs::File::open("tests/test.xlsx").unwrap();
    let data = traversal(file, 100).unwrap();
    assert_eq!(data.len(), 100);
}
