#![allow(dead_code)]
fn len_curve(n: i32) -> f64 {
    let m = (1. / n as f64).powi(2);
    (0..n).fold(0., |acc, x| {
        // step_i = √((1/n)^2 + ((2i+1)/n^2)^2)
        acc + (m + ((2 * x + 1) as f64 * m).powi(2)).sqrt()
    })
}

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-6;
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
    }
    assert_eq!(true, inrange);
}

fn dotest(n: i32, exp: f64) -> () {
    assert_fuzzy_equals(len_curve(n), exp);
}

#[test]
fn basic_tests_len_curve() {
    dotest(1, 1.414213562);
    dotest(10, 1.478197397);
    dotest(40, 1.478896272);
    dotest(200, 1.478940994);
    dotest(1200, 1.478942805);
}
