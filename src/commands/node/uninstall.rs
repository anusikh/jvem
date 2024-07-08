use crate::utils::file_utils::{get_installation_dir, run_command};

#[cfg(target_os = "windows")]
fn uninstall_util(version: &str) {
    let output = run_command(
        "powershell",
        vec![
            "-Command",
            &format!("rm -r {}", get_installation_dir(&version, "node")),
        ],
    );
    if output.status.success() {
        println!("uninstall successful");
    } else {
        println!("uninstall failed: maybe the mentioned node version is not installed locally");
    }
}

#[cfg(target_os = "linux")]
fn uninstall_util(version: &str) {
    let output = run_command(
        "rm",
        vec![
            "-rf",
            &format!("{}", get_installation_dir(&version, "node")),
        ],
    );
    if output.status.success() {
        println!("uninstall successful ");
    } else {
        println!("uninstall failed: maybe the mentioned node version is not installed locally");
    }
}

#[cfg(target_os = "macos")]
fn uninstall_util(version: &str) {
    let output = run_command(
        "rm",
        vec![
            "-rf",
            &format!("{}", get_installation_dir(&version, "node")),
        ],
    );
    if output.status.success() {
        println!("uninstall successful ");
    } else {
        println!("uninstall failed: maybe the mentioned node version is not installed locally");
    }
}

pub fn uninstall(version: String) {
    let _ = uninstall_util(&version);
}
