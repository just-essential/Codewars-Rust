#![allow(dead_code)]
fn dig_pow(n: i64, p: i32) -> i64 {
    if n < 0 || p < 0 {
        return -1;
    }
    let sum = n.to_string().char_indices().fold(0, |acc, (i, c)| {
        acc + (c.to_digit(10).unwrap() as i64).pow(i as u32 + p as u32)
    });
    if sum % n == 0 {
        sum / n
    } else {
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
