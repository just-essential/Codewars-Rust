#![allow(dead_code)]

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

use std::collections::HashMap;
use Direction::*;

use std::hash::{Hash, Hasher};

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as i8).hash(state)
    }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut result = Vec::new();
    let mut opposite_map = HashMap::new();
    opposite_map.insert(NORTH, SOUTH);
    opposite_map.insert(SOUTH, NORTH);
    opposite_map.insert(EAST, WEST);
    opposite_map.insert(WEST, EAST);
    for d in arr {
        if opposite_map.get(d) == result.last() {
            result.pop();
        } else {
            result.push(*d);
        }
    }
    result
}

#[test]
fn returns_expected() {
    use Direction::*;
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    assert_eq!(dir_reduc(&a), [WEST]);
    let a = [NORTH, WEST, SOUTH, EAST];
    assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
}
