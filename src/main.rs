use std::{
    env,
    fs::{self, File},
    io::{LineWriter, Write},
    process::exit,
};

use strum::EnumCount;
use strum_macros::EnumCount;

#[macro_use]
extern crate static_assertions;

#[derive(Debug, EnumCount)]
enum OpKind {
    Push,
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
            // Exhaustive handling of OpKinds in parse_word_as_op
            const_assert!(OpKind::COUNT == 3);
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

fn compile_program_darwin_arm64(program: Vec<Op>) {
    let file = File::create("out.s");
    if let Ok(file) = file {
        let mut file = LineWriter::new(file);
        let _ = file.write(b".global _start\n");
        let _ = file.write(b".align 2\n\n");
        let _ = file.write(b".text\n");
        let _ = file.write(b"print:\n");
        let _ = file.write(b"    adrp x0, num@PAGE\n");
        let _ = file.write(b"    add x0, x0, num@PAGEOFF\n");
        let _ = file.write(b"    ldr   x1, [sp], #16\n");
        let _ = file.write(b"    str x1, [x0]\n");
        let _ = file.write(b"    adrp x1, num@PAGE\n");
        let _ = file.write(b"    add x1, x1, num@PAGEOFF\n");
        let _ = file.write(b"    mov	X0, #1\n");
        let _ = file.write(b"    mov	X2, #3\n");
        let _ = file.write(b"    mov	X16, #4\n");
        let _ = file.write(b"    svc	#0x80\n");
        let _ = file.write(b"    ret\n\n");
        let _ = file.write(b"_start: \n");
        for op in program {
            match op.kind {
                OpKind::Push => {
                    if let Some(val) = op.value {
                        let _ = file.write(b"    // push \n");
                        let _ = file.write(format!("    mov x0, #{val}\n").as_bytes());
                        let _ = file.write("    str x0, [sp, #-16]!\n".to_string().as_bytes());
                    }
                }
                OpKind::Plus => {
                    let _ = file.write(b"    // plus \n");
                    let _ = file.write(b"    ldr   x0, [sp], #16\n");
                    let _ = file.write(b"    ldr   x1, [sp], #16\n");
                    let _ = file.write(b"    add   x3, x0, x1\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                }
                OpKind::Dump => {
                    let _ = file.write(b"    // dump \n");
                    let _ = file.write(b"    bl print\n");
                }
            }
            let _ = file.write(b"\n");
        }

        let _ = file.write(b"    // exit syscall\n");
        let _ = file.write(b"    mov x0, #0\n");
        let _ = file.write(b"    mov x16, #1\n");
        let _ = file.write(b"    svc #0x80\n");

        let _ = file.write(b".data\n");
        let _ = file.write(b"    num: .zero 8\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("ERROR: You have to pass in a mode and a file path.");
        eprintln!("Usage: ./rorth <mode> <filepath> ");
        eprintln!("    ./rorth sim <filepath> - Simulates the program.");
        eprintln!("    ./rorth com <filepath> - Compiles the program.");
        exit(1);
    }

    let mode = &args[1];
    let filename = &args[2];
    let lines = parse_file(filename.to_string());
    if let Ok(lines) = lines {
        let program = parse_word_as_op(lines);
        if mode == "sim" {
            simulate_program(program);
        } else if mode == "com" {
            compile_program_darwin_arm64(program);
        } else {
            eprintln!("ERROR: Unknown mode '{mode}'");
            exit(1);
        }
    } else {
        eprintln!("ERROR: Cannot read file: {filename}");
        exit(1);
    }
}
