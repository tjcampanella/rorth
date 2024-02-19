use std::{
    env,
    fs::{self, File},
    io::Write,
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[ERROR] You have to pass in a folder to test.");
        eprintln!("Usage: test [OPTIONS] <SUBCOMMAND> [ARGS]");
        eprintln!("  SUBCOMMAND:");
        eprintln!("    record       Record the test outputs.");
        exit(1);
    }

    let mut folder = &args[1];
    let mut record_flag = false;
    if args.len() > 2 {
        if &args[1] == "record" {
            record_flag = true;
            folder = &args[2];
        } else {
            eprintln!("Unknown subcommand: {}", &args[1]);
            exit(1);
        }
    }

    for entry in fs::read_dir(folder).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "rorth" {
                    let filename = path.file_name();

                    if let Some(filename) = filename {
                        let filename = filename.to_str().unwrap_or("");
                        let filename = format!("./examples/{filename}");
                        println!("[INFO] Simulating: {filename}");
                        let sim_output = std::process::Command::new("target/release/rorth")
                            .arg("sim")
                            .arg(&filename)
                            .output();

                        println!("[INFO] Compiling & Running: {filename}");
                        let com_output = std::process::Command::new("target/release/rorth")
                            .arg("com")
                            .arg("-r")
                            .arg("-s")
                            .arg(&filename)
                            .output();

                        let filename_pre = filename.split(".rorth").collect::<Vec<_>>()[0];
                        if let Ok(sim_output) = sim_output {
                            if let Ok(com_output) = com_output {
                                let sim_stdout =
                                    String::from_utf8_lossy(&sim_output.stdout).to_string();
                                let com_stdout =
                                    String::from_utf8_lossy(&com_output.stdout).to_string();
                                let sim_stderr =
                                    String::from_utf8_lossy(&sim_output.stderr).to_string();
                                let com_stderr =
                                    String::from_utf8_lossy(&com_output.stderr).to_string();
                                if !record_flag {
                                    assert!(sim_stderr.is_empty(), "[ERROR] {filename} simulation failed ❌.\nstderr: {sim_stderr}\n");
                                    assert!(com_stderr.is_empty(), "[ERROR] {filename} compilation failed ❌.\nstderr: {com_stderr}\n");
                                    assert!(sim_stdout == com_stdout, 
                                        "[ERROR] {filename} failed ❌. Simulation stdout bytes: {:?}\nSimulation stdout: \n{sim_stdout}\nCompilation stdout bytes: {:?}\nCompilation stdout: \n{com_stdout}\n", &sim_stdout, &com_stdout);
                                    let expected_filename = format!("{filename_pre}.txt");
									let expected = fs::read_to_string(&expected_filename);
                                    if let Ok(expected) = expected {
                                        assert!(com_stdout == expected, "[ERROR] Output does not match expected. ❌\n    Actual:\n{com_stdout}\n    Expected:\n{expected}");
                                    } else if let Err(error) = expected {
                                    eprintln!("Failed to read expected output file for {expected_filename}, {error:?}");
                                    exit(1);
                                    }
                                    println!("    {filename} passed ✅.");
                                } else {
                                    let record_file = File::create(format!("{filename_pre}.txt"));
                                    if let Ok(mut record_file) = record_file {
                                        let _ = record_file.write(&com_output.stdout);
                                        let _ = record_file.write(&com_output.stderr);
                                    }
                                }
                            } else {
                                eprintln!("[ERROR] Failed to compile program {filename}");
                                exit(1);
                            }
                        } else {
                            eprintln!("[ERROR] Failed to simulate program {filename}");
                            exit(1);
                        }
                    }
                }
            }
        }
    }
}
