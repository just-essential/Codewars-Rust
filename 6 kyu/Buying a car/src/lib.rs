#![allow(dead_code)]
fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut difference = (new - old) as f64;
    let mut amount = 0;
    let mut months = 0;
    let mut loss = perc / 100.;
    while (amount as f64) < difference {
        difference *= 1. - loss;
        amount += saving;
        months += 1;
        if months % 2 == 1 {
            loss += 0.005;
        }
    }
    (months, (amount as f64 - difference).round() as i32)
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
}

#[test]
fn basics_nb_months() {
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(12000, 8000, 1000, 1.5, (0, 4000));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
}
