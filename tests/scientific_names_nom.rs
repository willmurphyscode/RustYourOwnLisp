//example code from: https://github.com/Geal/nom at /tests/float.rs

#[macro_use]

extern crate nom;

use nom::{IResult,digit};

use std::str;

use std::str::FromStr;

struct Adjectives {
    adjs: Vec<String>
}

struct CoolName {
    value: String
}

struct CoolSound {
    value: String
}

struct Expression {
    prefix: Adjectives,
    name: CoolName,
    sound: CoolSound
}

named!(cool_name <&str>, 
  map_res!(
    recognize!(
      alt!(
        tag!("space") | tag!("time") | tag!("matter") | tag!("energy")
      )
    ),
    str::from_utf8
  ));

named!(cool_adjective, 
  recognize!(complete!(
  alt!(
        tag!("horrendous") | tag!("ultimate") | tag!("gargantuan")
  )) )
);

named!(space_or_comma <&str>, map_res!( alt!(tag!(" ") | tag!(", ")), str::from_utf8));

named!(read_adjective <&str>, map_res!(
  delimited!(space_or_comma, cool_adjective, space_or_comma),
  str::from_utf8
));


named!(adjectives < Vec<&str> >, many1!(read_adjective)); 

#[test]
fn cool_name_test() {
  assert_eq!(cool_name(&b"space"[..]), IResult::Done(&b""[..], "space")); 
}

#[test]
fn cool_adjective_test() {
  assert_eq!(cool_adjective(&b"horrendous"[..]), IResult::Done(&b""[..], &b"horrendous"[..]));
  assert_eq!(cool_adjective(&b"ultimate"[..]), IResult::Done(&b""[..], &b"ultimate"[..]));
  assert_eq!(cool_adjective(&b"gargantuan"[..]), IResult::Done(&b""[..], &b"gargantuan"[..]));
}

#[test]
fn adjectives_test() {
  let res = IResult::Done(&b""[..], vec!["horrendous", "ultimate", "gargantuan"]);
  let foo = adjectives(&b" horrendous, ultimate, gargantuan"[..]); 
  println!("{:?}", foo);
  assert_eq!(adjectives(&b"horrendous, ultimate, gargantuan"[..]), res);
}
