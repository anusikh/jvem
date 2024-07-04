use std::process::Command;

pub fn current() {
    let output_res = Command::new("java").args(vec!["--version"]).output();
    match output_res {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(_) => {
            println!("failed: jdk not set");
        }
    }
}
