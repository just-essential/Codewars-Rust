#![allow(dead_code)]

fn camel_case(str: &str) -> String {
    str.split_whitespace()
        .map(|word| word[..1].to_ascii_uppercase().to_owned() + &word[1..])
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn sample_test() {
    assert_eq!(camel_case("test case"), "TestCase");
    assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
    assert_eq!(camel_case("say hello "), "SayHello");
    assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
    assert_eq!(camel_case(""), "");
}
