use crate::utils::file_utils::{
    check_jdk_exists, check_path_exists, get_home_dir, get_installation_dir, run_command,
};

#[cfg(target_os = "windows")]
async fn usev_util(name: String) {
    let java_path = get_installation_dir(&name, "java");

    let java_home_future = tokio::spawn(async move {
        println!("setting JAVA_HOME...");

        let java_home_output = run_command(
            "powershell",
            vec![
                "-Command",
                &format!("[System.Environment]::SetEnvironmentVariable('JAVA_HOME','{}',[System.EnvironmentVariableTarget]::User)
                ", get_installation_dir(&name, "java"))
            ],
        );

        if java_home_output.status.success() {
            println!("set JAVA_HOME successfully");
        } else {
            println!(
                "error while setting JAVA_HOME: {}",
                String::from_utf8_lossy(&java_home_output.stderr)
            )
        }
    });

    let alias_future = tokio::spawn(async move {
        println!("creating symlink...");

        let _ = std::fs::remove_dir_all(&format!("{}/.jvem/java", get_home_dir()));

        let res =
            std::os::windows::fs::symlink_file(java_path, format!("{}/.jvem/java", get_home_dir()));

        match res {
            Ok(_) => println!("set jdk version successfully"),
            Err(e) => println!("failed: {}", e.to_string()),
        };
    });

    let (alias_task, java_home_task) = tokio::join!(alias_future, java_home_future);

    match (alias_task, java_home_task) {
        (Ok(alias_task), Ok(java_home_task)) => {
            println!("set jdk version successfully");
        }
        _ => println!("something went wrong"),
    }
}

#[cfg(target_os = "linux")]
async fn usev_util(name: String) {
    // remove the previously linked folder
    let _ = std::fs::remove_dir_all(&format!("{}/.jvem/java", get_home_dir()));

    let res = std::os::unix::fs::symlink(
        get_installation_dir(&name, "java"),
        format!("{}/.jvem/java", get_home_dir()),
    );

    match res {
        Ok(_) => println!("set jdk version successfully"),
        Err(e) => println!("failed: {}", e.to_string()),
    };
}

#[cfg(target_os = "macos")]
async fn usev_util(name: String) {
    let _ = std::fs::remove_dir_all(&format!("{}/.jvem/java", get_home_dir()));

    // check if Contents folder present inside extracted files (required for Graal VM support)
    let con_path = format!("{}/Contents", get_installation_dir(&name, "java"));
    let if_contents_exists = check_path_exists(&con_path);
    let final_path;

    if if_contents_exists == true {
        final_path = format!("{}/Contents/Home", get_installation_dir(&name, "java"));
    } else {
        final_path = format!("{}/Home", get_installation_dir(&name, "java"));
    };

    let res = std::os::unix::fs::symlink(final_path, format!("{}/.jvem/java", get_home_dir()));

    match res {
        Ok(_) => println!("set jdk version successfully"),
        Err(e) => println!("failed: {}", e.to_string()),
    };
}

pub async fn usev(name: String) {
    match check_jdk_exists(&name) {
        true => usev_util(name).await,
        false => println!("install the jdk first"),
    }
}
