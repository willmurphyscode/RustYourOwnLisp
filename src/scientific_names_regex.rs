extern crate regex;
use regex::Regex;

/*
1. At least 1 "cool" adjective ("horrendous", "incredible", "monstrous", "ultimate" etc.)
2. A science-sounding intermediate noun ("space", "time", "matter", "energy")
3. An onomatopoeia that makes a cool sound ("boom", "kablooie", "swoosh")
*/

pub fn is_scientific_name(input: &str) -> bool {
    let re = Regex::new(r"(horrendous|monstrous|incredible|ultimate)+\s(space|time|matter|energy)\s(boom|kablooie|swoosh)").unwrap();

    re.is_match(input)

}

#[test]
fn horrendous_space_kablooie() {
    let test_string = "horrendous space kablooie";
    assert!(is_scientific_name(&test_string));
}