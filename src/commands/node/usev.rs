use crate::utils::file_utils::{
    check_node_exists, get_home_dir, get_installation_dir, run_command,
};

#[cfg(target_os = "windows")]
fn usev_util(version: String) {
    let node_path = get_installation_dir(&version, "node");

    println!("creating symlink...");

    // remove the previously linked folder
    let _ = run_command(
        "powershell",
        vec![
            "-Command",
            &format!("rm -r {}\\.jvem\\node", get_home_dir()),
        ],
    );
    let output = run_command(
        "powershell",
        vec![
            "-Command",
            &format!(
                "New-Item -Path {}\\.jvem\\node -ItemType Junction -Value {}",
                get_home_dir(),
                node_path
            ),
        ],
    );
    if output.status.success() {
        println!("done!");
        println!("set node version successfully");
    } else {
        println!("failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

#[cfg(target_os = "linux")]
fn usev_util(version: String) {
    // remove the previously linked folder
    let _ = run_command("rm", vec!["-rf", &format!("{}/.jvem/node", get_home_dir())]);

    let output = run_command(
        "sh",
        vec![
            "-c",
            &format!(
                "ln --symbolic {} {}/.jvem/node",
                get_installation_dir(&version, "node"),
                get_home_dir()
            ),
        ],
    );

    if output.status.success() {
        println!("set node version successfully");
    } else {
        println!("failed: {}", String::from_utf8_lossy(&output.stderr))
    }
}

#[cfg(target_os = "macos")]
fn usev_util(version: String) {
    let _ = run_command("rm", vec!["-rf", &format!("{}/.jvem/node", get_home_dir())]);

    let output = run_command(
        "sh",
        vec![
            "-c",
            &format!(
                "ln -s {} {}/.jvem/node",
                get_installation_dir(&version, "node"),
                get_home_dir()
            ),
        ],
    );

    if output.status.success() {
        println!("set node version successfully");
    } else {
        println!("failed: {}", String::from_utf8_lossy(&output.stderr))
    }
}

pub fn usev(version: String) {
    match check_node_exists(&version) {
        true => usev_util(version),
        false => println!("install the node version first"),
    }
}
