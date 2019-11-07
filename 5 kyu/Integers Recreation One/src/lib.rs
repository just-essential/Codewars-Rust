#![allow(dead_code)]
use std::f64;

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n + 1)
        .map(|number| {
            (
                number,
                divisors(number).iter().fold(0, |acc, x| acc + x * x),
            )
        })
        .filter(|(_, sum)| (*sum as f64).sqrt().fract() <= f64::EPSILON)
        .collect()
}

fn divisors(number: u64) -> Vec<u64> {
    let mut result = Vec::new();
    for n in 1..((number as f64).sqrt() as u64 + 1) {
        if number % n == 0 {
            result.push(n);
            if n * n != number {
                result.push(number / n);
            }
        }
    }
    result.sort();
    result
}

fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
}
