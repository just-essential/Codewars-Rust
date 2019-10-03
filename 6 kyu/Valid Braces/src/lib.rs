#![allow(dead_code)]

fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

fn expect_false(s: &str) {
    assert_eq!(valid_braces(s), false);
}

fn expect_true(s: &str) {
    assert_eq!(valid_braces(s), true);
}

#[test]
fn basic_tests() {
    expect_true("()");
    expect_false("[(])");
    expect_false(")(}{][");
}
