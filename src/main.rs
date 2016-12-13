extern crate regex;
use regex::Regex;

//mod s_expression_types;
mod s_expression; 
mod opcode;

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


#[test]
fn test_s_expression_parse() {
    let vals = vec![Box::new(opcode::SExpression::atomic(1f64)),
        Box::new(opcode::SExpression::atomic(2f64))];
    let expected = opcode::SExpression::op(
        opcode::Operation { opcode: opcode::OpCode::Add, values: vals }
    );
    let input = &"( + 1 2)";
    let actual = s_expression::parse_SExpression(input).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_s_expression_eval() {
    let expected = 4f64; 
    let input = &"(+ 2 2)";
    let s = s_expression::parse_SExpression(input).unwrap();
    let actual = opcode::Visitor::visit_s_expression(&s);
    assert_eq!(expected, actual);
}

fn main() {

}
