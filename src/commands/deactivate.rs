use crate::utils::file_utils::{get_home_dir, run_command};

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
        println!("deactivation successful");
    } else {
        println!("deactivation failed");
    }
}

#[cfg(target_os = "linux")]
fn deactivate_util() {
    let output = run_command("rm", vec!["-rf", &format!("{}/.jvem/java", get_home_dir())]);
    if output.status.success() {
        println!("deactivation successful ");
    } else {
        println!("deactivation failed");
    }
}

pub fn deactivate() {
    let _ = deactivate_util();
}
