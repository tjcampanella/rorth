use std::{env, fs, process::exit};

#[derive(Debug)]
enum OpKind {
    Push,
    Pop,
    Plus,
    Dump,
}

#[derive(Debug)]
struct Op {
    kind: OpKind,
    value: Option<u32>,
}

fn parse_file(filename: String) -> Result<Vec<String>, ()> {
    let contents = fs::read_to_string(filename);
    if let Ok(contents) = contents {
        let lines = contents
            .split('\n')
            .map(std::string::ToString::to_string)
            .collect();
        return Ok(lines);
    }

    Err(())
}

fn parse_word_as_op(lines: Vec<String>) -> Vec<Op> {
    let mut result: Vec<Op> = vec![];
    for line in lines {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        for word in words {
            if let Ok(num) = word.parse::<u32>() {
                result.push(Op {
                    kind: OpKind::Push,
                    value: Some(num),
                });
            } else if word == "+" {
                result.push(Op {
                    kind: OpKind::Plus,
                    value: None,
                });
            } else if word == "dump" {
                result.push(Op {
                    kind: OpKind::Dump,
                    value: None,
                });
            } else {
                panic!("Unimplemented word: {word}")
            }
        }
    }

    result
}

fn simulate_program(program: Vec<Op>) {
    let mut stack = vec![];
    for op in program {
        match op.kind {
            OpKind::Push => {
                if let Some(val) = op.value {
                    stack.push(val);
                } else {
                    unreachable!();
                }
            }
            OpKind::Pop => todo!("Pop is not implemented yet"),
            OpKind::Plus => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a + b);
                    }
                }
            }
            OpKind::Dump => {
                if let Some(a) = stack.pop() {
                    println!("{a}");
                }
            }
        }
    }
}

fn compile_program(_program: Vec<Op>) {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: You have to pass in a file path.");
        exit(1);
    }

    let filename = &args[1];
    let lines = parse_file(filename.to_string());
    if let Ok(lines) = lines {
        let program = parse_word_as_op(lines);
        simulate_program(program);
    }
}
