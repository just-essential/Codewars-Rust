#![allow(dead_code)]

fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut result = Vec::new();
    result.push(n - 1);
    let mut remain = 2 * n - 1;
    loop {
        let mut x = *result.last().unwrap();
        while x > 1 && remain > 0 {
            for i in (1..x).rev() {
                if remain >= i * i {
                    x = i;
                    break;
                }
            }
            remain -= x * x;
            result.push(x);
        }
        if remain == 0 {
            break;
        }
        result.pop().map(|v| remain += v * v);
        result.last_mut().map(|v| {
            remain += 2 * (*v) - 1;
            *v -= 1
        });
        if result.is_empty() {
            return None;
        }
    }
    result.reverse();
    Some(result)
}

fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1, 3, 5, 8, 49]));
    testing(44, Some(vec![2, 3, 5, 7, 43]));
    testing(625, Some(vec![2, 5, 8, 34, 624]));
    testing(5, Some(vec![3, 4]));
}
