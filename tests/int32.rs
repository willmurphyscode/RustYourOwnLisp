//based on example code from: https://github.com/Geal/nom at /tests/float.rs

#[macro_use]

extern crate nom;


use nom::{IResult,digit};



use std::str;

use std::str::FromStr;



named!(unsigned_int <i32>, map_res!(

  map_res!(

    recognize!(

       alt_complete!(

        digit

      )

    ),
    str::from_utf8
  ),

  FromStr::from_str

));



named!(int <i32>, map!(

  pair!(

    opt!(alt!(tag!("+") | tag!("-"))),

    unsigned_int

  ),

  |(sign, value): (Option<&[u8]>, i32)| {

    sign.and_then(|s| if s[0] == ('-' as u8) { Some(-1i32) } else { None }).unwrap_or(1i32) * value

  }

));



#[test]

fn unsigned_int_test() {

  assert_eq!(unsigned_int(&b"123"[..]), IResult::Done(&b""[..], 123));

  assert_eq!(unsigned_int(&b"12"[..]), IResult::Done(&b""[..], 12));

  assert_eq!(unsigned_int(&b"120"[..]), IResult::Done(&b""[..], 120));

  assert_eq!(unsigned_int(&b"123"[..]), IResult::Done(&b""[..], 123));

  assert_eq!(unsigned_int(&b"1023"[..]), IResult::Done(&b""[..], 1023));

}



#[test]

fn int_test() {

  assert_eq!(int(&b"123"[..]),  IResult::Done(&b""[..], 123));

  assert_eq!(int(&b"+123456"[..]), IResult::Done(&b""[..], 123456));

  assert_eq!(int(&b"-123456"[..]), IResult::Done(&b""[..], -123456));

}