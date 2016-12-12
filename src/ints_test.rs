pub mod ints; 

#[test]
fn calculator1() {
    assert!(ints::parse_Term("22").is_ok());
    assert!(ints::parse_Term("(22)").is_ok());
    assert!(ints::parse_Term("((((22))))").is_ok());
    assert!(ints::parse_Term("((22)").is_err());
}
