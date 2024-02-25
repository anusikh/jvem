use super::utils::{check_jdk_exists, get_installation_dir, run_command};

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
        // remove the previously linked folder
        let _ = run_command(
            "powershell",
            vec!["-Command", &format!("rm -r \\Users\\anusi\\.jvem\\java")],
        );
        let output = run_command(
            "powershell",
            vec![
                "-Command",
                &format!(
                    "New-Item -Path C:\\Users\\anusi\\.jvem\\java -ItemType Junction -Value {}",
                    java_path
                ),
            ],
        );
        if output.status.success() {
            println!("set jdk version successfully");
        } else {
            println!("failed: {}", String::from_utf8_lossy(&output.stderr));
        }
    });

    let _ = tokio::join!(alias_future, java_home_future);

    println!("reopen powershell for changes to be reflected");
}

#[cfg(target_os = "linux")]
pub async fn usev_util(name: String) {
    use super::utils::get_home_dir;

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