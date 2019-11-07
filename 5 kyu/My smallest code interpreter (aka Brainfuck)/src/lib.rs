#![allow(dead_code)]
use std::collections::HashMap;

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.as_bytes();
    let mut input = input.into_iter();
    let mut data = vec![0 as u8];
    let mut output = Vec::new();
    let mut data_pointer = 0;
    let mut instruction_pointer = 0;
    let jump_table = get_jump_table(code);
    while instruction_pointer < code.len() {
        match code[instruction_pointer] {
            b'>' => {
                data_pointer += 1;
                if data_pointer == data.len() {
                    data.push(0);
                }
            }
            b'<' => data_pointer -= 1,
            b'+' => data[data_pointer] = data[data_pointer].wrapping_add(1),
            b'-' => data[data_pointer] = data[data_pointer].wrapping_sub(1),
            b'.' => output.push(data[data_pointer]),
            b',' => data[data_pointer] = input.next().unwrap(),
            b'[' => {
                if data[data_pointer] == 0 {
                    instruction_pointer = jump_table[&instruction_pointer];
                }
            }
            b']' => {
                if data[data_pointer] != 0 {
                    instruction_pointer = jump_table[&instruction_pointer];
                }
            }
            _ => (),
        }
        instruction_pointer += 1;
    }
    output
}

fn get_jump_table(code: &[u8]) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    let mut stack = Vec::new();
    for (i, c) in code.iter().enumerate() {
        match c {
            b'[' => {
                stack.push(i);
            }
            b']' => {
                let j = stack.pop().unwrap();
                result.insert(i, j);
                result.insert(j, i);
            }
            _ => (),
        }
    }
    result
}

// the function ez_vec takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
// Without it, character-based tests are a pain
fn ez_vec(string: &str, terminating_byte: u8) -> Vec<u8> {
    let mut result = string.as_bytes().to_vec();
    result.push(terminating_byte);
    result
}

#[test]
fn example_tests() {
    // Echo until byte 255 encountered
    assert_eq!(
        String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(),
        "Codewars"
    );
    // Echo until byte 0 encountered
    assert_eq!(
        String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(),
        "Codewars"
    );
    // Multiply two numbers
    assert_eq!(
        brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]),
        vec![72]
    );
}
