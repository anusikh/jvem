use crate::utils::file_utils::{check_jdk_exists, get_home_dir, get_installation_dir, run_command};

#[cfg(target_os = "windows")]
async fn usev_util(name: String) {
    let java_path = get_installation_dir(&name);

    let java_home_future = tokio::spawn(async move {
        println!("setting JAVA_HOME...");

        let java_home_output = run_command(
            "powershell",
            vec![
                "-Command",
                &format!("[System.Environment]::SetEnvironmentVariable('JAVA_HOME','{}',[System.EnvironmentVariableTarget]::User)
                ", get_installation_dir(&name))
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

        // remove the previously linked folder
        let _ = run_command(
            "powershell",
            vec![
                "-Command",
                &format!("rm -r {}\\.jvem\\java", get_home_dir()),
            ],
        );
        let output = run_command(
            "powershell",
            vec![
                "-Command",
                &format!(
                    "New-Item -Path {}\\.jvem\\java -ItemType Junction -Value {}",
                    get_home_dir(),
                    java_path
                ),
            ],
        );
        if output.status.success() {
            println!("done!");
        } else {
            println!("failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    });

    let _ = tokio::join!(alias_future, java_home_future);

    println!("set jdk version successfully");
}

#[cfg(target_os = "linux")]
pub async fn usev_util(name: String) {
    // remove the previously linked folder
    let _ = run_command("rm", vec!["-rf", &format!("{}/.jvem/java", get_home_dir())]);

    let output = run_command(
        "sh",
        vec![
            "-c",
            &format!(
                "ln --symbolic {} {}/.jvem/java",
                get_installation_dir(&name),
                get_home_dir()
            ),
        ],
    );

    if output.status.success() {
        println!("set jdk version successfully");
    } else {
        println!("failed: {}", String::from_utf8_lossy(&output.stderr))
    }
}

#[cfg(target_os = "macos")]
pub async fn usev_util(name: String) {
    use crate::utils::file_utils::check_path_exists;

    let _ = run_command("rm", vec!["-rf", &format!("{}/.jvem/java", get_home_dir())]);

    // check if Contents folder present inside extracted files (required for Graal VM support)
    let con_path = format!("{}/Contents", get_installation_dir(&name));
    let if_contents_exists = check_path_exists(&con_path);
    let final_path;

    if if_contents_exists == true {
        final_path = format!("{}/Contents/Home", get_installation_dir(&name));
    } else {
        final_path = format!("{}/Home", get_installation_dir(&name));
    };

    let output = run_command(
        "sh",
        vec![
            "-c",
            &format!("ln -s {} {}/.jvem/java", final_path, get_home_dir()),
        ],
    );

    if output.status.success() {
        println!("set jdk version successfully");
    } else {
        println!("failed: {}", String::from_utf8_lossy(&output.stderr))
    }
}

pub async fn usev(name: String) {
    match check_jdk_exists(&name) {
        true => {
            usev_util(name).await;
        }
        false => {
            println!("install the jdk first")
        }
    }
}
