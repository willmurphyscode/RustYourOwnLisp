extern crate regex;
use regex::Regex;
use name_types::CoolAdjective; 

mod opcode;
mod tokenize;

pub mod scientific_names;
pub mod name_types;


#[test]
fn cool_adjective_test() {
    assert!(scientific_names::parse_CoolAdjective(&"horrendous").is_ok());
    assert!(scientific_names::parse_CoolAdjective(&"horrendous, ").is_ok());
    assert_eq!(scientific_names::parse_CoolAdjective(&"gargantuan").unwrap(), CoolAdjective::Gargantuan);
}

#[test]
fn cool_noun_test() {
    assert_eq!(scientific_names::parse_CoolNoun(&"space").unwrap(), name_types::CoolNoun::Space);
}

#[test]
fn final_noun_test() {
    assert_eq!(scientific_names::parse_FinalNoun(&"device").unwrap(), name_types::FinalNoun::Device);
}

#[test]
fn adjective_list_test() {
    let expected = true; 
    let actual = scientific_names::parse_AdjectiveList(&"horrendous, gargantuan").is_ok();
    assert_eq!(expected, actual);
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
