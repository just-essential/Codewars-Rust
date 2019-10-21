#![allow(dead_code)]

use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                (".-", "A"),
                ("-...", "B"),
                ("-.-.", "C"),
                ("-..", "D"),
                (".", "E"),
                ("..-.", "F"),
                ("--.", "G"),
                ("....", "H"),
                ("..", "I"),
                (".---", "J"),
                ("-.-", "K"),
                (".-..", "L"),
                ("--", "M"),
                ("-.", "N"),
                ("---", "O"),
                (".--.", "P"),
                ("--.-", "Q"),
                (".-.", "R"),
                ("...", "S"),
                ("-", "T"),
                ("..-", "U"),
                ("...-", "V"),
                (".--", "W"),
                ("-..-", "X"),
                ("-.--", "Y"),
                ("--..", "Z"),
                ("-----", "0"),
                (".----", "1"),
                ("..---", "2"),
                ("...--", "3"),
                ("....-", "4"),
                (".....", "5"),
                ("-....", "6"),
                ("--...", "7"),
                ("---..", "8"),
                ("----.", "9"),
            ]
            .iter()
            .map(|&(key, value)| (key.to_string(), value.to_string()))
            .collect(),
        }
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        let mut codes = vec![];
        let mut time_unit = i32::max_value();
        let mut count = 0;
        let mut iter = encoded.trim_matches('0').chars().peekable();
        while let Some(c) = iter.next() {
            count += 1;
            if iter.peek() != Some(&c) {
                time_unit = time_unit.min(count);
                codes.push((c, count));
                println!("{}, {}", c, count);
                count = 0;
            }
        }
        let mut result = String::new();
        let (mid, long) = (3 * time_unit, 7 * time_unit);
        for (c, count) in codes.into_iter() {
            match c {
                '0' => {
                    if count == mid {
                        result.push(' ');
                    } else if count == long {
                        result.push_str("   ");
                    }
                }
                '1' => {
                    if count == mid {
                        result.push('-');
                    } else {
                        result.push('.');
                    }
                }
                _ => (),
            }
        }
        result
    }

    fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|codes| {
                codes
                    .split_whitespace()
                    .filter_map(|code| self.morse_code.get(code).cloned())
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn examples() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}
