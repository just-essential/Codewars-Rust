#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;

fn mix(s1: &str, s2: &str) -> String {
    let mut map = HashMap::<char, (usize, usize)>::new();
    let mut groups = Vec::new();
    for ch in s1.chars() {
        if ch.is_ascii_lowercase() {
            map.entry(ch).and_modify(|e| e.0 += 1).or_insert((1, 0));
        }
    }
    for ch in s2.chars() {
        if ch.is_ascii_lowercase() {
            map.entry(ch).and_modify(|e| e.1 += 1).or_insert((0, 1));
        }
    }
    for (key, value) in map.iter() {
        if value.0 > 1 || value.1 > 1 {
            let (from, count) = match value.0.cmp(&value.1) {
                Ordering::Greater => ('1', value.0),
                Ordering::Equal => ('=', value.0),
                Ordering::Less => ('2', value.1),
            };
            groups.push((from, count, key));
        }
    }
    groups.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0).then(a.2.cmp(&b.2))));
    groups
        .iter()
        .map(|&(from, count, key)| from.to_string() + ":" + &key.to_string().repeat(count))
        .collect::<Vec<_>>()
        .join("/")
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {
    testing(
        "Are they here",
        "yes, they are here",
        "2:eeeee/2:yy/=:hh/=:rr",
    );
    testing(
        "looping is fun but dangerous",
        "less dangerous than coding",
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
    );
    testing(
        " In many languages",
        " there's a pair of functions",
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
    );
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing(
        "A generation must confront the looming ",
        "codewarrs",
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
    );
}
