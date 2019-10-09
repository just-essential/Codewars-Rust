// Preloaded:
//
// struct MorseDecoder {
//     morse_code: HashMap<String, String>,
// }
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }
}
