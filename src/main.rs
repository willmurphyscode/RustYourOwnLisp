extern crate regex;
use regex::Regex;

mod s_expression; 
pub mod opcode;

#[test]
fn test_f64_parse() {
    let expected = -1.234f64;
    let actual = s_expression::parse_Num(&"-1.234").unwrap();

    assert_eq!(expected, actual);
    
    let expected_positive_whole = 3f64; 
    let actual_positive_whole = s_expression::parse_Num(&"3").unwrap();

    assert_eq!(expected_positive_whole, actual_positive_whole);

    let expected_negative_whole = -7f64;
    let actual_negative_whole = s_expression::parse_Num(&"-7").unwrap();

    assert_eq!(expected_negative_whole, actual_negative_whole);

}

fn main() {

}
