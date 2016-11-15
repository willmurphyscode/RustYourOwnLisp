extern crate regex;
use regex::Regex;

use std::io;
use std::io::Write;

mod opcode;
mod tokenize;
mod scientific_names_regex;

fn main() {
    println!("Official and reliable scientific names tester, wersion 32 alpha 4..");

        //get a reference to stdin 
    let stdin = io::stdin();

    //create a mutable string to hold what the user types. 
    let mut buffer = String::new();

    loop { 
        println!("Enter a string to test");
        print!(">>>");

        //manually flush stdout so that ">>>" prompt appears 
        io::stdout().flush().expect("Could not flush stdout. Panic!");
   
        stdin.read_line(&mut buffer).expect("Could not read stdin. Panic!");

        let is_name = scientific_names_regex::is_scientific_name(&buffer); 
        if is_name {
            println!("That is a decent scientific name. $3, please.");
        } else {
            println!("That is a terrible name. Try again.");
        }

       

        //clear the input string; only echo the most recent input. 
        buffer.clear();
    }   
}
