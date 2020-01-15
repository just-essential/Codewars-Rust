#![allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Ast {
    BinOp(String, Box<Ast>, Box<Ast>),
    UnOp(String, i32),
}

impl Ast {
    fn is_constant(&self) -> bool {
        if let Ast::UnOp(op, _) = self {
            op == "imm"
        } else {
            false
        }
    }

    fn simplify(&self) -> Ast {
        if let Ast::BinOp(op, a, b) = self {
            let (mut a, mut b) = (a.as_ref().clone(), b.as_ref().clone());
            if !a.is_constant() {
                a = a.simplify();
            }
            if !b.is_constant() {
                b = b.simplify();
            }
            if let (Ast::UnOp(op1, a), Ast::UnOp(op2, b)) = (a.clone(), b.clone()) {
                if op1 == "imm" && op2 == "imm" {
                    return Ast::UnOp(
                        "imm".to_owned(),
                        match op.as_str() {
                            "+" => a + b,
                            "-" => a - b,
                            "*" => a * b,
                            "/" => a / b,
                            _ => panic!("Invalid operation"),
                        },
                    );
                }
            }
            Ast::BinOp(op.to_owned(), Box::new(a), Box::new(b))
        } else {
            self.clone()
        }
    }

    fn to_instructions(&self) -> Vec<String> {
        let mut instructions = vec![];
        match self {
            Ast::UnOp(op, v) => instructions.push(op[..2].to_uppercase() + " " + &v.to_string()),
            Ast::BinOp(op, a, b) => {
                if let Ast::UnOp(_, _) = a.as_ref() {
                    instructions.append(&mut b.to_instructions());
                    instructions.push("SW".to_owned());
                    instructions.append(&mut a.to_instructions());
                } else {
                    instructions.append(&mut a.to_instructions());
                    instructions.push("PU".to_owned());
                    instructions.append(&mut b.to_instructions());
                    instructions.push("SW".to_owned());
                    instructions.push("PO".to_owned());
                }
                instructions.push(match op.as_str() {
                    "+" => "AD".to_owned(),
                    "-" => "SU".to_owned(),
                    "*" => "MU".to_owned(),
                    "/" => "DI".to_owned(),
                    _ => panic!("Invalid operation"),
                })
            }
        }
        instructions
    }
}

struct Compiler {}

impl Compiler {
    fn new() -> Compiler {
        Compiler {}
    }

    fn tokenize<'a>(&self, program: &'a str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];

        let mut iter = program.chars().peekable();
        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    }
                    '0'..='9' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    }
                    ' ' => {
                        iter.next();
                    }
                    _ => {
                        tokens.push(iter.next().unwrap().to_string());
                    }
                },
                None => break,
            }
        }

        tokens
    }

    fn compile(&mut self, program: &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    fn pass1(&mut self, program: &str) -> Ast {
        let tokens = self.tokenize(program);
        let mut ast_stack = vec![];
        let mut op_stack: Vec<char> = vec![];
        let mut args = std::collections::HashMap::new();
        let mut iter = tokens.split(|s| s == "]");
        let mut arg_list = iter.next().unwrap().iter();
        arg_list.next();
        let mut count = 0;
        while let Some(arg) = arg_list.next() {
            args.insert(arg, count);
            count += 1;
        }
        let mut expression = iter.next().unwrap().iter().rev();
        while let Some(term) = expression.next() {
            let ch = term.chars().next().unwrap();
            match ch {
                'a'..='z' | 'A'..='Z' => {
                    let &index = args.get(term).unwrap();
                    ast_stack.push(Ast::UnOp("arg".to_owned(), index));
                }
                '0'..='9' => {
                    ast_stack.push(Ast::UnOp("imm".to_owned(), term.parse().unwrap()));
                }
                '(' => {
                    while let Some(op) = op_stack.pop() {
                        if op == ')' {
                            break;
                        }
                        let a = ast_stack.pop().unwrap();
                        let b = ast_stack.pop().unwrap();
                        ast_stack.push(Ast::BinOp(op.to_string(), Box::new(a), Box::new(b)));
                    }
                }
                ')' => op_stack.push(')'),
                _ => {
                    while !op_stack.is_empty()
                        && *op_stack.last().unwrap() != ')'
                        && (ch == '+' || ch == '-')
                        && (*op_stack.last().unwrap() == '*' || *op_stack.last().unwrap() == '/')
                    {
                        let a = ast_stack.pop().unwrap();
                        let b = ast_stack.pop().unwrap();
                        ast_stack.push(Ast::BinOp(
                            op_stack.pop().unwrap().to_string(),
                            Box::new(a),
                            Box::new(b),
                        ));
                    }
                    op_stack.push(ch);
                }
            }
        }
        while let Some(op) = op_stack.pop() {
            let a = ast_stack.pop().unwrap();
            let b = ast_stack.pop().unwrap();
            ast_stack.push(Ast::BinOp(op.to_string(), Box::new(a), Box::new(b)));
        }
        ast_stack.pop().unwrap()
    }

    fn pass2(&mut self, ast: &Ast) -> Ast {
        ast.simplify()
    }

    fn pass3(&mut self, ast: &Ast) -> Vec<String> {
        ast.to_instructions()
    }
}

fn simulate(assembly: Vec<String>, argv: Vec<i32>) -> i32 {
    let mut r = (0, 0);
    let mut stack: Vec<i32> = vec![];
    for ins in assembly {
        let mut ws = ins.split_whitespace();
        match ws.next() {
            Some("IM") => r.0 = i32::from_str_radix(ws.next().unwrap(), 10).unwrap(),
            Some("AR") => r.0 = argv[i32::from_str_radix(ws.next().unwrap(), 10).unwrap() as usize],
            Some("SW") => r = (r.1, r.0),
            Some("PU") => stack.push(r.0),
            Some("PO") => r.0 = stack.pop().unwrap(),
            Some("AD") => r.0 += r.1,
            Some("SU") => r.0 -= r.1,
            Some("MU") => r.0 *= r.1,
            Some("DI") => r.0 /= r.1,
            _ => panic!("Invalid instruction encountered"),
        }
    }
    r.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass1() {
        assert_eq!(
            Compiler::new().pass1("[ x y ] ( x + y ) / 2"),
            Ast::BinOp(
                "/".to_string(),
                Box::new(Ast::BinOp(
                    "+".to_string(),
                    Box::new(Ast::UnOp("arg".to_string(), 0)),
                    Box::new(Ast::UnOp("arg".to_string(), 1))
                )),
                Box::new(Ast::UnOp("imm".to_string(), 2))
            )
        );
    }

    #[test]
    fn pass2() {
        assert_eq!(
            Compiler::new().pass2(&Ast::BinOp(
                "+".to_string(),
                Box::new(Ast::UnOp("arg".to_string(), 0)),
                Box::new(Ast::BinOp(
                    "*".to_string(),
                    Box::new(Ast::UnOp("imm".to_string(), 2)),
                    Box::new(Ast::UnOp("imm".to_string(), 5))
                ))
            )),
            Ast::BinOp(
                "+".to_string(),
                Box::new(Ast::UnOp("arg".to_string(), 0)),
                Box::new(Ast::UnOp("imm".to_string(), 10))
            )
        );
    }

    #[test]
    fn pass3() {
        assert_eq!(
            Compiler::new().pass3(&Ast::BinOp(
                "+".to_string(),
                Box::new(Ast::UnOp("arg".to_string(), 0)),
                Box::new(Ast::UnOp("imm".to_string(), 10))
            )),
            vec!["IM 10", "SW", "AR 0", "AD"]
        )
    }

    #[test]
    fn simulator() {
        assert_eq!(simulate(vec!["IM 7".to_owned()], vec![3]), 7);
        assert_eq!(simulate(vec!["AR 1".to_owned()], vec![1, 2, 3]), 2);
    }
}
