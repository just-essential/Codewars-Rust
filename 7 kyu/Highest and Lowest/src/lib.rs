#![allow(dead_code)]

fn high_and_low(numbers: &str) -> String {
    let mut numbers = numbers.split_whitespace();
    let mut min = numbers.next().unwrap().parse::<i32>().unwrap();
    let mut max = min;
    while let Some(number) = numbers.next() {
        let number = number.parse::<i32>().unwrap();
        min = min.min(number);
        max = max.max(number);
    }
    format!("{} {}", max, min)
}

#[test]
fn sample_test() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}
