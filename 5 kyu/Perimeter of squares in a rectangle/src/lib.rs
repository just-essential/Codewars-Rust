#![allow(dead_code)]
fn perimeter(n: u64) -> u64 {
    (0..n)
        .fold((1, 1, 1), |acc, _| (acc.1, acc.0 + acc.1, acc.2 + acc.1))
        .2
        * 4
}

fn dotest(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
    dotest(5, 80);
    dotest(7, 216);
    dotest(20, 114624);
    dotest(30, 14098308);
}
