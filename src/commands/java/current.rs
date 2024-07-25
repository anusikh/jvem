use crate::utils::file_utils::run_command;

pub fn current() {
    let res = run_command("java", vec!["--version"]);
    if res.status.success() {
        println!("java version: {}", String::from_utf8_lossy(&res.stdout));
    } else {
        println!("failed: node not set");
    }
}
