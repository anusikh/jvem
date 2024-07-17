use crate::utils::file_utils::{check_node_exists, get_home_dir, run_command, get_installation_dir};

#[cfg(target_os = "windows")]
fn usev_util(version: String) {
    let node_path = get_installation_dir(&version, "node");

    println!("creating symlink...");
    let _ = std::fs::remove_dir_all(&format!("{}/.jvem/node", get_home_dir()));

    let res = run_command(
        "powershell",
        vec![
            "-Command",
            &format!("New-Item -Path {}\\.jvem\\node -ItemType Junction -Value {}", get_home_dir(), node_path),
        ],
    );

    if res.status.success() {
        println!("done!");
    } else {
        println!("failed: {}", String::from_utf8_lossy(&res.stderr));
    }
}

#[cfg(target_os = "linux")]
fn usev_util(version: String) {
    // remove the previously linked folder
    let _ = std::fs::remove_dir_all(&format!("{}/.jvem/node", get_home_dir()));

    let res = std::os::unix::fs::symlink(
        get_installation_dir(&version, "node"),
        format!("{}/.jvem/node", get_home_dir()),
    );

    match res {
        Ok(_) => println!("set node version successfully"),
        Err(e) => println!("failed: {}", e.to_string()),
    }
}

#[cfg(target_os = "macos")]
fn usev_util(version: String) {
    let _ = std::fs::remove_dir_all(&format!("{}/.jvem/node", get_home_dir()));

    let res = std::os::unix::fs::symlink(
        get_installation_dir(&version, "node"),
        format!("{}/.jvem/node", get_home_dir()),
    );

    match res {
        Ok(_) => println!("set node version successfully"),
        Err(e) => println!("failed: {}", e.to_string()),
    }
}

pub fn usev(version: String) {
    match check_node_exists(&version) {
        true => usev_util(version),
        false => println!("install the node version first"),
    }
}
