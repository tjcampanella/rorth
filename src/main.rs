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

#[derive(Debug, EnumCount, PartialEq, Clone, Copy)]
enum OpKind {
    Push,
    If,
    While,
    Do,
    End,
    Plus,
    Minus,
    Print,
    Equals,
    Dup,
    Swap,
    Rot,
    Drop,
    Over,
    GT,
    LT,
}

#[derive(Debug, Clone, Copy)]
struct Op {
    kind: OpKind,
    value: Option<u64>,
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
            const_assert!(OpKind::COUNT == 16);
            if let Ok(num) = word.parse::<u64>() {
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
            } else if word == "over" {
                result.push(Op {
                    kind: OpKind::Over,
                    value: None,
                });
            } else if word == "if" {
                result.push(Op {
                    kind: OpKind::If,
                    value: None,
                });
            } else if word == "while" {
                result.push(Op {
                    kind: OpKind::While,
                    value: None,
                });
            } else if word == "do" {
                result.push(Op {
                    kind: OpKind::Do,
                    value: None,
                });
            } else if word == "end" {
                result.push(Op {
                    kind: OpKind::End,
                    value: None,
                });
            } else if word == ">" {
                result.push(Op {
                    kind: OpKind::GT,
                    value: None,
                });
            } else if word == "<" {
                result.push(Op {
                    kind: OpKind::LT,
                    value: None,
                });
            } else {
                panic!("Unknown word: {word}")
            }
        }
    }

    result
}

fn cross_reference_blocks(program: &mut Vec<Op>, ip_start: usize) {
    let mut ip = ip_start;
    let mut curr_if = None;
    let mut curr_if_ip = 0;
    let mut curr_while = None;
    let mut curr_while_ip = 0;
    let mut curr_do = None;
    let mut curr_do_ip = 0;
    while ip < program.len() {
        let mut op = program[ip];
        // Exhaustive handling of Ops in cross_reference_blocks. Remember not all need to be
        // accounted for here only Ops that form blocks.
        const_assert!(OpKind::COUNT == 16);
        if op.kind == OpKind::If {
            if curr_if.is_none() && op.value.is_none() {
                curr_if = Some(op);
                curr_if_ip = ip;
            } else if curr_if.is_some() && op.value.is_none() {
                cross_reference_blocks(program, ip);
            }
        } else if op.kind == OpKind::While {
            if curr_while.is_none() && op.value.is_none() {
                curr_while = Some(op);
                curr_while_ip = ip;
            } else if curr_while.is_some() && op.value.is_none() {
                cross_reference_blocks(program, ip);
            }
        }
        if op.kind == OpKind::Do {
            if curr_do.is_none() && op.value.is_none() {
                curr_do = Some(op);
                curr_do_ip = ip;
            } else if curr_do.is_some() && op.value.is_none() {
                cross_reference_blocks(program, ip);
            }
        } else if op.kind == OpKind::End {
            if let Some(mut if_op) = curr_if {
                if if_op.value.is_none() && op.value.is_none() {
                    if_op.value = (ip + 1).try_into().ok();
                    op.value = ip.try_into().ok();
                    program[curr_if_ip] = if_op;
                    program[ip] = op;
                    curr_if = None;
                    curr_if_ip = 0;
                }
            }

            if let Some(mut while_op) = curr_while {
                if let Some(mut do_op) = curr_do {
                    if while_op.value.is_none() && op.value.is_none() {
                        while_op.value = Some(0);
                        op.value = curr_while_ip.try_into().ok();
                        program[ip] = op;
                        program[curr_while_ip] = while_op;
                        curr_while = None;
                        curr_while_ip = 0;

                        if do_op.value.is_none() {
                            do_op.value = (ip + 1).try_into().ok();
                            program[curr_do_ip] = do_op;
                            curr_do = None;
                            curr_do_ip = 0;
                        }
                    }
                }
            }
        }
        ip += 1;
    }
}

fn simulate_program(program: &[Op]) {
    let mut stack = vec![];
    let mut ip = 0;
    while ip < program.len() {
        let op = &program[ip];
        match op.kind {
            OpKind::Push => {
                if let Some(val) = op.value {
                    stack.push(val);
                }
                ip += 1;
            }
            OpKind::Plus => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a + b);
                    }
                }
                ip += 1;
            }
            OpKind::Minus => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b - a);
                    }
                }
                ip += 1;
            }
            OpKind::Equals => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((b == a).into());
                    }
                }
                ip += 1;
            }
            OpKind::Print => {
                if let Some(a) = stack.pop() {
                    let mut a = format!("{a}");
                    if a.len() < 20 {
                        let num_to_pad = 20 - a.len();
                        for _ in 0..num_to_pad {
                            a.insert(0, '\0');
                        }
                    }
                    println!("{a}");
                }
                ip += 1;
            }
            OpKind::Dup => {
                if let Some(a) = stack.pop() {
                    stack.push(a);
                    stack.push(a);
                }
                ip += 1;
            }
            OpKind::Swap => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(a);
                        stack.push(b);
                    }
                }
                ip += 1;
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
                ip += 1;
            }
            OpKind::Drop => {
                stack.pop();
                ip += 1;
            }
            OpKind::Over => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push(b);
                        stack.push(a);
                        stack.push(b);
                    }
                }
                ip += 1;
            }
            OpKind::If | OpKind::Do => {
                if let Some(a) = stack.pop() {
                    if a == 1 {
                        ip += 1;
                    } else if let Some(ind) = op.value {
                        if let Ok(ind) = ind.try_into() {
                            ip = ind;
                        }
                    }
                }
            }
            OpKind::While => {
                ip += 1;
            }
            OpKind::End => {
                if let Some(ind) = op.value {
                    if let Ok(ind) = ind.try_into() {
                        if ind != ip {
                            ip = ind;
                            continue;
                        }
                    }
                }
                ip += 1;
            }
            OpKind::GT => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((b > a).into());
                    }
                }
                ip += 1;
            }
            OpKind::LT => {
                if let Some(a) = stack.pop() {
                    if let Some(b) = stack.pop() {
                        stack.push((b < a).into());
                    }
                }
                ip += 1;
            }
        }
    }
}

fn compile_program_darwin_arm64(program: &[Op], filename: &str) {
    let file = File::create(format!("{filename}.s"));
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
        let _ = file.write(b"    mov x3, #19\n");
        let _ = file.write(b"convert_loop:\n");
        let _ = file.write(b"    udiv x4, x1, x2\n");
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
        let _ = file.write(b"    mov x2, #20\n");
        let _ = file.write(b"    mov x16, #4\n");
        let _ = file.write(b"    svc #0x80\n");
        let _ = file.write(b"    adrp x0, newline@PAGE\n");
        let _ = file.write(b"    add x0, x0, newline@PAGEOFF\n");
        let _ = file.write(b"    mov x1, x0\n");
        let _ = file.write(b"    mov x0, #1\n");
        let _ = file.write(b"    mov x2, #1\n");
        let _ = file.write(b"    mov x16, #4 \n");
        let _ = file.write(b"    svc #0x80\n");
        let _ = file.write(b"	mov x1, #20\n");
        let _ = file.write(b"loop:\n");
        let _ = file.write(b"	strb wzr, [x4], #1\n");
        let _ = file.write(b"	subs x1, x1, #1\n");
        let _ = file.write(b"	bne loop\n");
        let _ = file.write(b"	ret\n\n");
        let _ = file.write(b"_start: \n");

        let mut ip = 0;
        while ip < program.len() {
            let op = &program[ip];
            match op.kind {
                OpKind::Push => {
                    if let Some(val) = op.value {
                        let _ = file.write(b"    // push \n");
                        let _ = file.write(format!("    ldr x0, ={val}\n").as_bytes());
                        let _ = file.write("    str x0, [sp, #-16]!\n".to_string().as_bytes());
                    }
                    ip += 1;
                }
                OpKind::Plus => {
                    let _ = file.write(b"    // plus \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    add x3, x0, x1\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::Minus => {
                    let _ = file.write(b"    // minus \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    sub x3, x1, x0\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::Equals => {
                    let _ = file.write(b"    // equals \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    cmp x0, x1\n");
                    let _ = file.write(b"    cset w0, EQ\n");
                    let _ = file.write(b"    str w0, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::Dup => {
                    let _ = file.write(b"    // dup \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::Swap => {
                    let _ = file.write(b"    // swap \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                    let _ = file.write(b"    str x1, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::Rot => {
                    let _ = file.write(b"    // rot \n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    ldr x2, [sp], #16\n");
                    let _ = file.write(b"    ldr x3, [sp], #16\n");
                    let _ = file.write(b"    str x2, [sp, #-16]!\n");
                    let _ = file.write(b"    str x1, [sp, #-16]!\n");
                    let _ = file.write(b"    str x3, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::Drop => {
                    let _ = file.write(b"    // drop \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    ip += 1;
                }
                OpKind::Print => {
                    let _ = file.write(b"    // print \n");
                    let _ = file.write(b"    bl print\n");
                    ip += 1;
                }
                OpKind::Over => {
                    let _ = file.write(b"    // over \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    str x1, [sp, #-16]!\n");
                    let _ = file.write(b"    str x0, [sp, #-16]!\n");
                    let _ = file.write(b"    str x1, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::If => {
                    let _ = file.write(b"    // if \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    cmp x0, #0\n");
                    if let Some(ind) = op.value {
                        let _ = file.write(format!("    beq addr_{ind}\n").as_bytes());
                    }
                    ip += 1;
                }
                OpKind::While => {
                    let _ = file.write(format!("addr_{ip}:\n").as_bytes());
                    ip += 1;
                }
                OpKind::Do => {
                    let _ = file.write(b"    // do \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    cmp x0, #0\n");
                    if let Some(ind) = op.value {
                        let _ = file.write(format!("    beq addr_{ind}\n").as_bytes());
                    }
                    ip += 1;
                }
                OpKind::End => {
                    if let Some(ind) = op.value {
                        if let Ok(ind) = TryInto::<usize>::try_into(ind) {
                            if ind != ip {
                                let _ = file.write(format!("    b addr_{ind}\n").as_bytes());
                            }
                        }
                    }
                    ip += 1;
                    let _ = file.write(format!("addr_{ip}:\n").as_bytes());
                }
                OpKind::GT => {
                    let _ = file.write(b"    // > \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    cmp x1, x0\n");
                    let _ = file.write(b"    cset w0, GE\n");
                    let _ = file.write(b"    str w0, [sp, #-16]!\n");
                    ip += 1;
                }
                OpKind::LT => {
                    let _ = file.write(b"    // < \n");
                    let _ = file.write(b"    ldr x0, [sp], #16\n");
                    let _ = file.write(b"    ldr x1, [sp], #16\n");
                    let _ = file.write(b"    cmp x1, x0\n");
                    let _ = file.write(b"    cset w0, LT\n");
                    let _ = file.write(b"    str w0, [sp, #-16]!\n");
                    ip += 1;
                }
            }
            let _ = file.write(b"\n");
        }
        let _ = file.write(b"    // exit syscall\n");
        let _ = file.write(b"    mov x0, #0\n");
        let _ = file.write(b"    mov x16, #1\n");
        let _ = file.write(b"    svc #0x80\n\n");
        let _ = file.write(b".data\n");
        let _ = file.write(b"    num: .zero 20\n");
        let _ = file.write(b"    newline: .asciz \"\\n\" \n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("ERROR: You have to pass in a mode and a file path.");
        eprintln!("Usage: rorth [OPTIONS] <SUBCOMMAND> [ARGS]");
        eprintln!("  SUBCOMMAND:");
        eprintln!("    sim <file>            Simulate the program");
        eprintln!("    com [OPTIONS] <file>  Compile the program");
        eprintln!("      OPTIONS:");
        eprintln!("        -r                  Run the program after successful compilation");
        eprintln!("        -s                  Silence all logging statements.");
        exit(1);
    }

    let mode = &args[1];
    let mut filename = &args[2];
    let mut run_flag = false;
    let mut silence_flag = false;
    if args.len() > 3 {
        if &args[2] == "-r" {
            run_flag = true;
        } else if &args[2] == "-s" {
            silence_flag = true;
        } else {
            eprintln!("ERROR: Unknown option: {}", &args[2]);
            eprintln!("Usage: rorth [OPTIONS] <SUBCOMMAND> [ARGS]");
            eprintln!("  SUBCOMMAND:");
            eprintln!("    sim <file>            Simulate the program");
            eprintln!("    com [OPTIONS] <file>  Compile the program");
            eprintln!("      OPTIONS:");
            eprintln!("        -r                  Run the program after successful compilation");
            eprintln!("        -s                  Silence all logging statements.");
            exit(1);
        }
        filename = &args[3];
    }

    if args.len() > 4 {
        if &args[2] != "-r" && &args[2] != "-s" {
            eprintln!("ERROR: Unknown option: {}", &args[2]);
            eprintln!("Usage: rorth [OPTIONS] <SUBCOMMAND> [ARGS]");
            eprintln!("  SUBCOMMAND:");
            eprintln!("    sim <file>            Simulate the program");
            eprintln!("    com [OPTIONS] <file>  Compile the program");
            eprintln!("      OPTIONS:");
            eprintln!("        -r                  Run the program after successful compilation");
            eprintln!("        -s                  Silence all logging statements.");
            exit(1);
        }

        if &args[3] != "-r" && &args[3] != "-s" {
            eprintln!("ERROR: Unknown option: {}", &args[3]);
            eprintln!("Usage: rorth [OPTIONS] <SUBCOMMAND> [ARGS]");
            eprintln!("  SUBCOMMAND:");
            eprintln!("    sim <file>            Simulate the program");
            eprintln!("    com [OPTIONS] <file>  Compile the program");
            eprintln!("      OPTIONS:");
            eprintln!("        -r                  Run the program after successful compilation");
            eprintln!("        -s                  Silence all logging statements.");
            exit(1);
        }

        if &args[2] == "-r" {
            run_flag = true;
        } else if &args[2] == "-s" {
            silence_flag = true;
        }

        if &args[3] == "-r" {
            run_flag = true;
        } else if &args[3] == "-s" {
            silence_flag = true;
        }

        filename = &args[4];
    }

    let lines = parse_file(filename.to_string());
    if let Ok(lines) = lines {
        let mut program = parse_word_as_op(lines);
        cross_reference_blocks(&mut program, 0);
        if mode == "sim" {
            simulate_program(&program);
        } else if mode == "com" {
            let filename_pre: Vec<&str> = filename.split(".rorth").collect();
            let filename_pre = filename_pre[0];
            compile_program_darwin_arm64(&program, filename_pre);
            if run_flag {
                if !silence_flag {
                    println!("[CMD] as -arch arm64 -o {filename_pre}.o {filename_pre}.s");
                }
                let res = std::process::Command::new("as")
                    .arg("-arch")
                    .arg("arm64")
                    .arg("-o")
                    .arg(format!("{filename_pre}.o"))
                    .arg(format!("{filename_pre}.s"))
                    .status();

                if let Ok(as_status) = res {
                    if as_status.success() {
                        if !silence_flag {
                            println!("[CMD] ld -o {filename_pre} {filename_pre}.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64");
                        }
                        let res = std::process::Command::new("ld")
                            .arg("-o")
                            .arg(filename_pre)
                            .arg(format!("{filename_pre}.o"))
                            .arg("-L")
                            .arg("/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib")
                            .arg("-lSystem")
                            .arg("-syslibroot")
                            .arg("`xcrun -sdk macosx --show-sdk-path`")
                            .arg("-e")
                            .arg("_start")
                            .arg("-arch")
                            .arg("arm64")
                            .status();

                        if let Ok(ld_status) = res {
                            if ld_status.success() {
                                if !silence_flag {
                                    println!("[CMD] ./{filename_pre}");
                                }
                                let mut res =
                                    std::process::Command::new(format!("./{filename_pre}")).spawn();

                                if let Ok(ref mut res) = res {
                                    let _ = res.wait();
                                }

                                if let Some(err) = res.err() {
                                    eprintln!("ERROR: Failed to execute compiled program: {err}");
                                }
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("ERROR: Unknown mode '{mode}'");
            exit(1);
        }
    } else {
        eprintln!("ERROR: Cannot read file: {filename}");
        exit(1);
    }
}
