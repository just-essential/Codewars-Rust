#![allow(dead_code)]
fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let mut result = -1;
    fn combination(t: i32, k: usize, ls: &[i32], acc: i32, result: &mut i32) {
        if acc > t {
            return;
        }
        if k == 1 {
            for i in ls {
                let sum = acc + i;
                if sum <= t && sum > *result {
                    *result = sum;
                }
            }
        } else if ls.len() == k {
            let sum = acc + ls.iter().sum::<i32>();
            if sum <= t && sum > *result {
                *result = sum;
            }
        } else if ls.len() > k {
            for i in 0..ls.len() - k + 1 {
                combination(t, k - 1, &ls[i + 1..], acc + ls[i], result);
            }
        }
    }
    combination(t, k as usize, &ls[..], 0, &mut result);
    result
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

#[test]
fn basics_choose_best_sum() {
    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);
}
