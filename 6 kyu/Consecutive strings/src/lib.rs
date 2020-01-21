#![allow(dead_code)]
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k <= 0 {
        String::new()
    } else {
        // strarr
        //     .windows(k)
        //     .map(|arr| arr.join(""))
        //     .rev()
        //     .max_by_key(|s| s.len())
        //     .unwrap_or(String::new())
        strarr
            .windows(k)
            .rev()
            .max_by_key(|arr| arr.iter().map(|s| s.len()).sum::<usize>())
            .map(|arr| arr.join(""))
            .unwrap_or(String::new())
    }
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(
        vec!["zone", "abigail", "theta", "form", "libe", "zas"],
        2,
        "abigailtheta",
    );
    testing(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
        "oocccffuucccjjjkkkjyyyeehh",
    );
    testing(vec![], 3, "");
    testing(
        vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
        3,
        "ixoyx3452zzzzzzzzzzzz",
    );
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
