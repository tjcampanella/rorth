use std::{fs, process::exit};

fn main() {
    for entry in fs::read_dir("./examples").unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "rorth" {
                    let filename = path.file_name();

                    if let Some(filename) = filename {
                        let filename = filename.to_str().unwrap_or("");
                        let filename = format!("./examples/{filename}");
                        println!("Simulating: {filename}");
                        let sim_output = std::process::Command::new("target/release/rorth")
                            .arg("sim")
                            .arg(&filename)
                            .output();

                        println!("Compiling: {filename}");
                        let com_output = std::process::Command::new("target/release/rorth")
                            .arg("com")
                            .arg("-r")
                            .arg("-s")
                            .arg(&filename)
                            .output();

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
                                assert!(sim_stderr.is_empty(), "Simulation stderr: {sim_stderr}\n");
                                assert!(com_stderr.is_empty(), "Compilation stderr: {com_stderr}\n");
                                assert!(sim_stdout == com_stdout, 
                                        "Simulation stdout bytes: {:?}\nSimulation stdout: \n{sim_stdout}\nCompilation stdout bytes: {:?}\nCompilation stdout: \n{com_stdout}\n", &sim_stdout, &com_stdout);
                            } else {
                                eprintln!("Failed to compile program {filename}");
                                exit(1);
                            }
                        } else {
                            eprintln!("Failed to simulate program {filename}");
                            exit(1);
                        }
                    }
                }
            }
        }
    }
}
