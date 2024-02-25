use super::utils::{get_installation_dir, run_command};

#[cfg(target_os = "windows")]
fn uninstall_util(name: &str) {
    let output = run_command(
        "powershell",
        vec![
            "-Command",
            &format!("rm -r {}", get_installation_dir(&name)),
        ],
    );
    if output.status.success() {
        println!("uninstall successful ");
    } else {
        println!(
            "uninstall failed: {} ",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[cfg(target_os = "linux")]
fn uninstall_util(name: &str) {
    let output = run_command(
        "rm",
        vec!["-rf", &format!("{}", get_installation_dir(&name))],
    );
    if output.status.success() {
        println!("uninstall successful ");
    } else {
        println!(
            "uninstall failed: {} ",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn uninstall(name: String) {
    let _ = uninstall_util(&name);
}
