#![allow(dead_code)]
fn order_weight(s: &str) -> String {
    let mut v = s
        .split_ascii_whitespace()
        .map(|num| (num.bytes().map(|d| (d - b'0') as i32).sum::<i32>(), num))
        .collect::<Vec<_>>();
    v.sort_unstable();
    v.iter()
        .map(|(_, num)| *num)
        .collect::<Vec<_>>()
        .join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
