extern crate regex;
use regex::Regex;
use name_types::CoolAdjective; 

mod opcode;
mod tokenize;

pub mod ints; 

pub mod scientific_names;
pub mod name_types;

#[test]
fn calculator1() {
    assert!(ints::parse_Term("22").is_ok());
    assert!(ints::parse_Term("(22)
    ").is_ok());
    assert!(ints::parse_Term("((((22))))").is_ok());
    assert!(ints::parse_Term("((22)").is_err());
}

#[test]
fn cool_adjective_test() {
    assert!(scientific_names::parse_coolAdjective("horrendous").is_ok());
}

fn main() {
    println!("Hello, world!");
    let c = '+';
    let code = opcode::OpCode::get_opcode(c);

    let input = String::from("+ 2 3");

    let tokens = tokenize::Token::tokenize(input);

    for tok in &tokens {
        println!("{:?}", tok);
    }

    let char_from_code = match code {
        Some(c) => opcode::OpCode::get_char(c),
        None => ' '  
    };

    println!("Opcode was {:?}", code);
    println!("Returned char was {}", char_from_code);
}
