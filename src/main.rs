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
    Minus,
    Print,
    Equals,
    Dup,
    Swap,
    Rot,
    Drop,
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
    for mut line in lines {
        let comment = line.find("//");
        if let Some(comment_ind) = comment {
            line = line.chars().take(comment_ind).collect();
        }
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        for word in words {
            // Exhaustive handling of OpKinds in parse_word_as_op
            const_assert!(OpKind::COUNT == 9);
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
            } else if word == "-" {
                result.push(Op {
                    kind: OpKind::Minus,
                    value: None,
                });
            } else if word == "print" {
                result.push(Op {
                    kind: OpKind::Print,
                    value: None,
                });
            } else if word == "=" {
                result.push(Op {
                    kind: OpKind::Equals,
                    value: None,
                });
            } else if word == "dup" {
                result.push(Op {
                    kind: OpKind::Dup,
                    value: None,
                });
            } else if word == "swap" {
                result.push(Op {
                    kind: OpKind::Swap,
                    value: None,
                });
            } else if word == "rot" {
                result.push(Op {
                    kind: OpKind::Rot,
                    value: None,
                });
            } else if word == "drop" {
                result.push(Op {
                    kind: OpKind::Drop,
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
            OpKind::Minus => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b - a);
                    }
                }
            }
            OpKind::Equals => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((b == a).into());
                    }
                }
            }
            OpKind::Print => {
                if let Some(a) = stack.pop() {
                    println!("{a}");
                }
            }
            OpKind::Dup => {
                if let Some(a) = stack.pop() {
                    stack.push(a);
                    stack.push(a);
                }
            }
            OpKind::Swap => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a);
                        stack.push(b);
                    }
                }
            }
            OpKind::Rot => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        if let Some(c) = stack.pop() {
                            stack.push(b);
                            stack.push(a);
                            stack.push(c);
                        }
                    }
                }
            }
            OpKind::Drop => {
                stack.pop();
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
        let _ = file.write(b"    mov x2, #10\n");
        let _ = file.write(b"    mov x3, #7\n");
        let _ = file.write(b"convert_loop:\n");
        let _ = file.write(b"    sdiv x4, x1, x2\n");
        let _ = file.write(b"    mul x5, x4, x2\n");
        let _ = file.write(b"    sub x6, x1, x5 \n");
        let _ = file.write(b"    and w6, w6, #0xFF\n");
        let _ = file.write(b"    add x6, x6, #'0'\n");
        let _ = file.write(b"    strb w6, [x0, x3]\n");
        let _ = file.write(b"    sub x3, x3, #1\n");
        let _ = file.write(b"    mov x1, x4\n");
        let _ = file.write(b"    cmp x1, #0\n");
        let _ = file.write(b"    bne convert_loop\n");
        let _ = file.write(b"    adrp x4, num@PAGE\n");
        let _ = file.write(b"    add x4, x4, num@PAGEOFF\n");
        let _ = file.write(b"    mov x1, x4\n");
        let _ = file.write(b"    mov x0, #1\n");
        let _ = file.write(b"    mov x2, #8\n");
        let _ = file.write(b"    mov x16, #4\n");
        let _ = file.write(b"    svc #0x80\n");
        let _ = file.write(b"    adrp x0, newline@PAGE\n");
        let _ = file.write(b"    add x0, x0, newline@PAGEOFF\n");
        let _ = file.write(b"    mov x1, x0\n");
        let _ = file.write(b"    mov x0, #1\n");
        let _ = file.write(b"    mov x2, #1\n");
        let _ = file.write(b"    mov x16, #4 \n");
        let _ = file.write(b"    svc #0x80\n");
        let _ = file.write(b"    mov x3, #0\n");
        let _ = file.write(b"    str x3, [x4]\n");
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
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    add x3, x0, x1\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                }
                OpKind::Minus => {
                    let _ = file.write(b"    // minus \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    sub x3, x1, x0\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                }
                OpKind::Equals => {
                    let _ = file.write(b"    // equals \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    cmp x0, x1\n");
                    let _ = file.write(b"    cset w0, EQ\n");
                    let _ = file.write(b"    str w0, [sp, #-16]!\n");
                }
                OpKind::Dup => {
                    let _ = file.write(b"    // dup \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                }
                OpKind::Swap => {
                    let _ = file.write(b"    // swap \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                    let _ = file.write(b"    str x1, [sp, #-16]!\n");
                }
                OpKind::Rot => {
                    let _ = file.write(b"    // rot \n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    ldr x2, [sp], #16\n");
                    let _ = file.write(b"    ldr x3, [sp], #16\n");
                    let _ = file.write(b"    str x2, [sp, #-16]!\n");
                    let _ = file.write(b"    str x1, [sp, #-16]!\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                }
                OpKind::Drop => {
                    let _ = file.write(b"    // drop \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                }
                OpKind::Print => {
                    let _ = file.write(b"    // print \n");
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
        let _ = file.write(b"    newline: .asciz \"\\n\" \n");
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
