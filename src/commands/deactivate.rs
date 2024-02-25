use super::utils::{get_home_dir, run_command};

#[cfg(target_os = "windows")]
fn deactivate_util() {
    let output = run_command(
        "powershell",
        vec![
            "-Command",
            &format!("rm -r {}\\.jvem\\java", get_home_dir()),
        ],
    );
    if output.status.success() {
        println!("deactivation successful ");
    } else {
        println!(
            "deactivation failed: {} ",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[cfg(target_os = "linux")]
fn deactivate_util() {
    let output = run_command("rm", vec!["-rf", &format!("{}/.jvem/java", get_home_dir())]);
    if output.status.success() {
        println!("deactivation successful ");
    } else {
        println!(
            "deactivation failed: {} ",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn deactivate() {
    let _ = deactivate_util();
}