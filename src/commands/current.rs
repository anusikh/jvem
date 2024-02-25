use super::utils::run_command;

pub fn current() {
    let output = run_command("java", vec!["--version"]);

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout))
    } else {
        println!("failed: {}", String::from_utf8_lossy(&output.stderr))
    }
}
