#![allow(dead_code)]

use std::mem;

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }
    let mut result = Vec::new();
    let (mut long, mut width) = if lng > wdth { (lng, wdth) } else { (wdth, lng) };
    result.push(width);
    long -= width;
    while long > 0 {
        if long < width {
            mem::swap(&mut long, &mut width);
        }
        result.push(width);
        long -= width;
    }
    Some(result)
}

fn testing(lng: i32, wdth: i32, exp: Option<Vec<i32>>) -> () {
    assert_eq!(sq_in_rect(lng, wdth), exp)
}

#[test]
fn tests_sq_in_rect() {
    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}
