#![allow(dead_code)]

fn next_bigger_number(n: i64) -> i64 {
    let mut numbers = n.to_string().as_bytes().to_vec();
    let mut i = numbers.len().checked_sub(2);
    while let Some(v) = i {
        if numbers[v + 1] > numbers[v] {
            break;
        }
        i = v.checked_sub(1);
    }
    if let Some(i) = i {
        let mut j = numbers.len() - 1;
        while numbers[j] <= numbers[i] {
            j -= 1;
        }
        numbers.swap(i, j);
        numbers[i + 1..].reverse();
        String::from_utf8(numbers).unwrap().parse().unwrap()
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
    }
}
