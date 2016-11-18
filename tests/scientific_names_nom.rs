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

named!(cool_name <CoolName>, map_res!(
  map_res!(
    recognize!(
      alt!(
        "space" | "time" | "matter" | "energy"
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));


named!(float <f32>, map!(
  pair!(
    opt!(alt!(tag!("+") | tag!("-"))),
    unsigned_float
  ),
  |(sign, value): (Option<&[u8]>, f32)| {
    sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1f32) } else { None }).unwrap_or(1f32) * value
  }
));



#[test]

fn adjective_test() {
    
  assert_eq!(unsigned_float(&b"123.456"[..]), IResult::Done(&b""[..], 123.456));

}



#[test]

fn float_test() {

  assert_eq!(float(&b"123.456"[..]),  IResult::Done(&b""[..], 123.456));

  assert_eq!(float(&b"+123.456"[..]), IResult::Done(&b""[..], 123.456));

  assert_eq!(float(&b"-123.456"[..]), IResult::Done(&b""[..], -123.456));

}