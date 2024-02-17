use std::fs;

fn main() {
    for entry in fs::read_dir("./examples").unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "rorth" {
                    let filename = path.file_name();

                    if let Some(filename) = filename {
                        println!("filename: {}", filename.to_str().unwrap_or(""));

                        let _sim_output = std::process::Command::new("cargo")
                            .arg("run")
                            .arg("--release")
                            .arg("sim")
                            .arg(filename)
                            .output();

                        let _com_output = std::process::Command::new("cargo")
                            .arg("run")
                            .arg("--release")
                            .arg("com")
                            .arg("-r")
                            .arg("-s")
                            .arg(filename)
                            .output();
                    }
                }
            }
        }
    }
}
