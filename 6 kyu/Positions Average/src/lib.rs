#![allow(dead_code)]
fn pos_average(s: &str) -> f64 {
    let strings = s.split(", ").map(|s| s.as_bytes()).collect::<Vec<_>>();
    let n = strings.len();
    let len = strings.get(0).map(|s| s.len()).unwrap_or(0);
    let mut count = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            for k in 0..len {
                if strings[i][k] == strings[j][k] {
                    count += 1;
                }
            }
        }
    }
    (100 * count) as f64 / ((n * (n - 1) / 2) * len) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_fuzzy(s: &str, expected: f64) {
        println!("{}", s);
        let merr = 1.0e-9;
        let actual = pos_average(s);
        let inrange = if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
        if inrange == false {
            println!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            );
        } else {
            println!("....... correct");
        }
        assert_eq!(inrange, true);
        println!("{}", inrange);
    }
    #[test]
    fn basic_tests() {
        assert_fuzzy(
            "466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096",
            26.6666666667,
        );
        assert_fuzzy(
            "444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490",
            29.2592592593,
        );
    }
}
