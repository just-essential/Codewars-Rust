#![allow(dead_code)]

fn row_sum_odd_numbers(n: i64) -> i64 {
    if n < 1 {
        0
    } else {
        n.pow(3)
    }
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
