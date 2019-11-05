#![allow(dead_code)]

use std::collections::BTreeMap;

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let max_number = l.iter().fold(0, |acc, x| acc.max(x.abs()));
    let mut prime_numbers = vec![];
    let mut is_prime = vec![true; max_number as usize + 1];
    let limit = (max_number as f64).sqrt() as usize + 1;
    for i in 2..=limit {
        if is_prime[i] {
            for j in (i * i..=max_number as usize).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    for i in 2..=max_number {
        if is_prime[i as usize] {
            prime_numbers.push(i);
        }
    }
    let mut result = BTreeMap::new();
    for i in l.iter() {
        for prime in prime_numbers.iter() {
            if *prime > i.abs() {
                break;
            }
            if i % prime == 0 {
                let sum = result.entry(prime).or_insert(0);
                *sum += i;
            }
        }
    }
    result
        .into_iter()
        .map(|(key, value)| (*key, value))
        .collect()
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {
    testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
    testing(
        vec![15, 21, 24, 30, 45],
        vec![(2, 54), (3, 135), (5, 90), (7, 21)],
    );
}
