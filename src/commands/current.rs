use crate::utils::file_utils::run_command;

pub fn current() {
    let output = run_command("java", vec!["--version"]);
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
