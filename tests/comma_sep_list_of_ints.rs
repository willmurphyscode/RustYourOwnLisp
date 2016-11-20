#[macro_use]
extern crate nom;
use nom::{IResult,digit};
use std::str;
use std::str::FromStr;

named!(split< &[u8], Vec< &[u8] > >, 
    separated_list!(char!(' '), digit)
);


#[test]
fn test_split() {
     assert_eq!(split(&b"123 1 2 3"[..]), IResult::Done(&b""[..], vec![&b"123"[..], &b"1"[..], &b"2"[..], &b"3"[..]]));
}