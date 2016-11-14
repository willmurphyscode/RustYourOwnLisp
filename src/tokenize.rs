extern crate regex;
use regex::Regex;
use opcode::OpCode; 

use opcode;

pub enum Token {
    Operation(OpCode),
    Value(i32),
    OpenParen,
    CloseParen
}

impl Token {
    pub fn tokenize(input: String) -> Vec<Token> {
        let mut retval : Vec<Token> = vec![];

        let match_number = Regex::new(r"^\d+$").unwrap();
        let match_open_paren = Regex::new(r"\(").unwrap();
        let match_close_paren = Regex::new(r"\)").unwrap();
        let match_opcode = Regex::new(r"[+-*/]").unwrap();

        let ary : Vec<&str> = input.split_whitespace().collect();

        for item in ary {
            let tok = Token::make_token(item);
            retval.push(tok);
        }

        return retval; 
    }

    pub fn eval(stack: Vec<Token>) -> i32 {
        let mut retval = 0; 
        
        while let Some(current) = stack.pop() {
            match current {
                Token::OpenParen => () 
            }
            
        } 
    }

    fn make_token(input: &str) -> Token {

        let match_number = Regex::new(r"^\d+$").unwrap();
        let match_open_paren = Regex::new(r"^\($").unwrap();
        let match_close_paren = Regex::new(r"^\)$").unwrap();
        let match_opcode = Regex::new(r"^[+-*/]$").unwrap();

        if match_number.is_match(input) {
            let number = input.parse::<i32>().expect("Failed with bad number matching regex"); 
            return Token::Value(number);
        }

        if match_open_paren.is_match(input) {
            return Token::OpenParen;
        }

        if match_close_paren.is_match(input) {
            return Token::CloseParen;
        }

        if match_opcode.is_match(input) {
            let ch = input.chars().nth(0).unwrap(); 
            let op = opcode::OpCode::get_opcode(ch).expect("bad opcode parsing regex");
            return Token::Operation(op);
        }

        let error = String::from("Error parsing") + input; 
        panic!(error);
    }

}