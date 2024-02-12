use std::fs;

#[derive(Debug)]
enum OpKind {
    PUSH,
    POP,
    PLUS,
    DUMP,
}

#[derive(Debug)]
struct Op {
    kind: OpKind,
    value: Option<u32>,
}

fn parse_file(filename: String) -> Result<Vec<String>, ()> {
    let contents = fs::read_to_string(filename);
    if let Ok(contents) = contents {
        let lines = contents.split('\n').map(std::string::ToString::to_string).collect();
        return Ok(lines);
    }

    Err(())
}

fn parse_word_as_op(lines: Vec<String>) -> Result<Vec<Op>, ()> {
    let mut result: Vec<Op> = vec![];
    for line in lines {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        for word in words {
            if let Ok(num) = word.parse::<u32>() {
                result.push(Op {
                    kind: OpKind::PUSH,
                    value: Some(num),
                });
            } else if word == "+" {
                result.push(Op {
                    kind: OpKind::PLUS,
                    value: None,
                });
            } else if word == "dump" {
                result.push(Op {
                    kind: OpKind::DUMP,
                    value: None,
                });
            } else {
                panic!("Unimplemented word: {word}")
            }
        }
    }

    Ok(result)
}

fn simulate_program(program: Vec<Op>) {
    let mut stack = vec![];
    for op in program {
        match op.kind {
            OpKind::PUSH => {
                if let Some(val) = op.value {
                    stack.push(val);
                } else {
                    unreachable!();
                }
            }
            OpKind::POP => todo!("Pop is not implemented yet"),
            OpKind::PLUS => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a + b);
                    }
                }
            }
            OpKind::DUMP => {
                if let Some(a) = stack.pop() {
                    println!("{a}");
                }
            }
        }
    }
}

fn compile_program(_program: Vec<Op>) {}

fn main() {
    let lines = parse_file("./examples/stack.rorth".to_string());
    if let Ok(lines) = lines {
        if let Ok(program) = parse_word_as_op(lines) {
            simulate_program(program);
        }
    }
}
