#![allow(dead_code)]
fn check_choose(m: u64, n: u64) -> i64 {
    let mut temp = 1;
    let mut x = -1;
    for i in 0..n {
        temp = temp * (n - i) / (i + 1);
        if temp >= m {
            if temp == m {
                x = i as i64 + 1;
            }
            break;
        }
    }
    x
}

fn dotest(m: u64, n: u64, exp: i64) -> () {
    assert_eq!(check_choose(m, n), exp)
}

#[test]
fn basics_check_choose() {
    dotest(6, 4, 2);
    dotest(4, 4, 1);
    dotest(35, 7, 3);
    dotest(36, 7, -1);
    dotest(184756, 20, 10);
}
