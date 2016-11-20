#[macro_use]
extern crate nom;
use nom::{IResult,digit};
use std::str;
use std::str::FromStr;



named!(split< &[u8], Vec< &[u8] > >, 
    separated_list!(char!(' '), digit)
);

fn int_from_byte_vec(input: &[u8]) -> i32 {
    let my_str = str::from_utf8(input).expect("Error: could not read bytes as str");
    let opt = my_str.parse::<i32>();
    match opt {
        Ok(v) => v,
        Err(_) => panic!("Could not parse input")
    }
}

fn int_vec_from_byte_ary_vec(input: Vec<&[u8]>) -> Vec<i32> {
    input.iter().map(|&x| int_from_byte_vec(x)).collect::<Vec<i32>>()
}
//map!(I -> IResult<I,O>, O -> P) => I -> IResult<I, P> maps a function on the result of a parser
// O -> P is the transform, take in an O, output a P, in this case take in a ref to a byte array and output an int.
// 
named!(map_bytes <&[u8], Vec<i32> >,
    map!(split, int_vec_from_byte_ary_vec)
);


#[test]
fn test_split() {
     assert_eq!(split(&b"123 1 2 3"[..]), IResult::Done(&b""[..], vec![&b"123"[..], &b"1"[..], &b"2"[..], &b"3"[..]]));
}

#[test]
fn test_int_from_byte_vec() {
    let input = &b"123"[..];
    let expected = 123; 
    let actual = int_from_byte_vec(input); 
    assert_eq!(expected, actual);
}

#[test]
fn test_map_split() {
    let input = &b"12 1 3"[..];
    let expected =  IResult::Done(&b""[..], vec![12, 1, 3]);
    let actual = map_bytes(input);
    assert_eq!(expected, actual);
}