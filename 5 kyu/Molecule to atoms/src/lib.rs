type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut result = Molecule::new();
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        let mut current;
        match c {
            '(' | '[' | '{' => {
                let match_parenthesis = if c == '(' {
                    ')'
                } else if c == '[' {
                    ']'
                } else {
                    '}'
                };
                let mut sub_s = String::new();
                let mut mismatched = true;
                while let Some(c) = iter.next() {
                    if c != match_parenthesis {
                        sub_s.push(c);
                    } else {
                        mismatched = false;
                        break;
                    }
                }
                if mismatched {
                    return Err(ParseError {});
                }
                current = if let Ok(sub) = parse_molecule(&sub_s) {
                    sub
                } else {
                    return Err(ParseError {});
                };
            }
            _ => {
                if c.is_ascii_uppercase() {
                    let mut atom = (String::new(), 1);
                    atom.0.push(c);
                    while iter.peek().map_or(false, |c| c.is_ascii_lowercase()) {
                        let c = iter.next().unwrap();
                        atom.0.push(c);
                    }
                    current = vec![atom];
                } else {
                    return Err(ParseError {});
                }
            }
        }
        let mut times = 0;
        while iter.peek().map_or(false, |c| c.is_ascii_digit()) {
            let c = iter.next().unwrap();
            times = times * 10 + (c as u8 - '0' as u8) as usize;
        }
        if times != 0 {
            for (_, i) in current.iter_mut() {
                *i *= times;
            }
        }
        for sub_i in current.iter_mut() {
            for result_i in result.iter_mut() {
                if sub_i.0 == result_i.0 {
                    result_i.1 += sub_i.1;
                    sub_i.1 = 0;
                    break;
                }
            }
        }
        for sub_i in current {
            if sub_i.1 != 0 {
                result.push(sub_i);
            }
        }
    }
    Ok(result)
}

#[test]
fn sample_test() {
    assert_eq!(
        parse_molecule("H2O").unwrap(),
        [("H".to_owned(), 2), ("O".to_owned(), 1)]
    );
    assert_eq!(
        parse_molecule("Mg(OH)2").unwrap(),
        [
            ("Mg".to_owned(), 1),
            ("O".to_owned(), 2),
            ("H".to_owned(), 2)
        ]
    );
    assert_eq!(
        parse_molecule("K4[ON(SO3)2]2").unwrap(),
        [
            ("K".to_owned(), 4),
            ("O".to_owned(), 14),
            ("N".to_owned(), 2),
            ("S".to_owned(), 4)
        ]
    );
    parse_molecule("pie").expect_err("Not a valid molecule");
    parse_molecule("Mg(OH").expect_err("Mismatched parenthesis");
    parse_molecule("Mg(OH}2").expect_err("Mismatched parenthesis");
}
