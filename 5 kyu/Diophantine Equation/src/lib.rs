#![allow(dead_code)]

fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    let n_sqrt = (n as f64).sqrt() as u64 + 1;
    for a in 1..n_sqrt {
        if n % a == 0 {
            let b = n / a;
            if (b - a) % 4 == 0 {
                result.push(((a + b) / 2, (b - a) / 4));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(solequa(n), exp)
    }

    #[test]
    fn basics_solequa() {
        testing(5, vec![(3, 1)]);
        testing(20, vec![(6, 2)]);
        testing(9001, vec![(4501, 2250)]);
        testing(9004, vec![(2252, 1125)]);
    }
}
