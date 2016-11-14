extern crate regex;
use regex::Regex;

mod opcode;
mod tokenize;

fn main() {
    println!("Hello, world!");
    let c = '+';
    let code = opcode::OpCode::get_opcode(c);

    


    let char_from_code = match code {
        Some(c) => opcode::OpCode::get_char(c),
        None => ' '  
    };

    println!("Opcode was {:?}", code);
    println!("Returned char was {}", char_from_code);
}
