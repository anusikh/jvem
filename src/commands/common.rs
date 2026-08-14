//! Shared implementations of the commands exposed by each runtime
//! (java, node, maven). Platform-specific behavior lives here and in
//! `utils::file_utils`, so the per-runtime command modules stay thin.

use std::{error::Error, fs};

use crate::runtime::Runtime;
use crate::utils::file_utils::{self, run_command};

/// A human-readable name for a version of a runtime, e.g. "jdk zulu17".
fn describe(runtime: Runtime, name: &str) -> String {
    match runtime {
        Runtime::Maven => "maven".to_string(),
        _ => format!("{} {}", runtime.label(), name),
    }
}

/// Install `name` of `runtime` from `link`.
pub fn install(runtime: Runtime, name: &str, link: &str) -> Result<(), Box<dyn Error>> {
    if file_utils::is_installed(runtime, name) {
        println!(
            "{} is already installed; if you believe this is wrong, run `clean` and try again",
            describe(runtime, name)
        );
        return Ok(());
    }
    file_utils::create_install_dir(runtime, name)?;
    let archive = file_utils::download_archive(runtime, name, link)?;
    file_utils::extract_archive(runtime, name, &archive)?;
    Ok(())
}

/// Remove an installed version.
pub fn uninstall(runtime: Runtime, name: &str) -> Result<(), Box<dyn Error>> {
    let dir = file_utils::install_dir(runtime, name);
    if !dir.exists() {
        println!("{} is not installed locally", describe(runtime, name));
        return Ok(());
    }
    fs::remove_dir_all(&dir)?;
    match runtime {
        Runtime::Maven => println!("maven uninstall successful"),
        _ => println!("uninstall successful"),
    }
    Ok(())
}

/// Point the runtime's symlink/junction at an installed version.
pub async fn use_version(runtime: Runtime, name: &str) -> Result<(), Box<dyn Error>> {
    if !file_utils::is_installed(runtime, name) {
        println!(
            "{} is not installed; install it first",
            describe(runtime, name)
        );
        return Ok(());
    }
    use_version_impl(runtime, name.to_string()).await
}

#[cfg(target_os = "windows")]
async fn use_version_impl(runtime: Runtime, name: String) -> Result<(), Box<dyn Error>> {
    let link = file_utils::symlink_path(runtime);
    let install = file_utils::install_dir(runtime, &name);
    let install_for_java_home = install.clone();

    let symlink_task = tokio::spawn(async move {
        println!("creating symlink...");
        let _ = fs::remove_dir_all(&link);
        let output = run_command(
            "powershell",
            &[
                "-Command",
                &format!(
                    "New-Item -Path {} -ItemType Junction -Value {}",
                    link.display(),
                    install.display()
                ),
            ],
        );
        match output {
            Ok(output) if output.status.success() => println!("done!"),
            Ok(output) => println!("failed: {}", String::from_utf8_lossy(&output.stderr).trim()),
            Err(e) => println!("failed: {e}"),
        }
    });

    if runtime == Runtime::Java {
        let java_home_task = tokio::spawn(async move {
            println!("setting JAVA_HOME...");
            let output = run_command(
                "powershell",
                &[
                    "-Command",
                    &format!(
                        "[System.Environment]::SetEnvironmentVariable('JAVA_HOME','{}',[System.EnvironmentVariableTarget]::User)",
                        install_for_java_home.display()
                    ),
                ],
            );
            match output {
                Ok(output) if output.status.success() => println!("set JAVA_HOME successfully"),
                Ok(output) => println!(
                    "error while setting JAVA_HOME: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ),
                Err(e) => println!("error while setting JAVA_HOME: {e}"),
            }
        });

        let _ = tokio::join!(symlink_task, java_home_task);
        println!("set jdk version successfully");
    } else {
        let _ = symlink_task.await;
    }
    Ok(())
}

#[cfg(unix)]
async fn use_version_impl(runtime: Runtime, name: String) -> Result<(), Box<dyn Error>> {
    let link = file_utils::symlink_path(runtime);
    let _ = fs::remove_dir_all(&link);

    let target = use_version_target(runtime, &name);
    std::os::unix::fs::symlink(&target, &link)?;
    println!("set {} version successfully", runtime.label());
    Ok(())
}

/// The directory a runtime's symlink should point at. On macos, JDK
/// bundles keep their files under `Contents/Home` (or `Home`).
#[cfg(unix)]
fn use_version_target(runtime: Runtime, name: &str) -> std::path::PathBuf {
    let dir = file_utils::install_dir(runtime, name);
    if runtime == Runtime::Java && std::env::consts::OS == "macos" {
        let contents = dir.join("Contents");
        if contents.is_dir() {
            return contents.join("Home");
        }
        return dir.join("Home");
    }
    dir
}

/// Show the currently active version of `runtime`.
pub async fn current(runtime: Runtime) -> Result<(), Box<dyn Error>> {
    match runtime {
        Runtime::Java => current_java(),
        Runtime::Node => current_node().await,
        Runtime::Maven => Err("maven has no `current` command".into()),
    }
}

fn current_java() -> Result<(), Box<dyn Error>> {
    let output = run_command("java", &["--version"])?;
    if output.status.success() {
        println!(
            "java version: {}",
            String::from_utf8_lossy(&output.stdout).trim_end()
        );
    } else {
        println!("failed: java not set");
    }
    Ok(())
}

/// Spawn a task that prints the version of a program, if it runs.
fn version_task(
    program: &'static str,
    args: &'static [&'static str],
    label: &'static str,
    fail_message: &'static str,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let output = run_command(program, args);
        match output {
            Ok(output) if output.status.success() => {
                println!(
                    "{label} version: {}",
                    String::from_utf8_lossy(&output.stdout).trim_end()
                );
            }
            _ => println!("{fail_message}"),
        }
    })
}

#[cfg(target_os = "windows")]
async fn current_node() -> Result<(), Box<dyn Error>> {
    let _ = tokio::join!(
        version_task(
            "powershell",
            &["-C", "node --version"],
            "node",
            "failed: node not set"
        ),
        version_task(
            "powershell",
            &["-C", "npm --version"],
            "npm",
            "failed: npm not set"
        ),
    );
    Ok(())
}

#[cfg(unix)]
async fn current_node() -> Result<(), Box<dyn Error>> {
    let _ = tokio::join!(
        version_task("node", &["--version"], "node", "failed: node not set"),
        version_task("npm", &["--version"], "npm", "failed: npm not set"),
    );
    Ok(())
}

/// List locally installed versions.
pub fn list_local(runtime: Runtime) -> Result<(), Box<dyn Error>> {
    let installed = file_utils::list_installed(runtime)?;
    if installed.is_empty() {
        println!("no installations found locally");
    } else {
        for version in installed {
            println!("{version}");
        }
    }
    Ok(())
}

/// Remove empty version directories left behind by failed installs.
pub fn clean(runtime: Runtime) -> Result<(), Box<dyn Error>> {
    file_utils::clean_empty_dirs(runtime)?;
    Ok(())
}

/// Remove the active-version symlink/junction.
pub fn deactivate(runtime: Runtime) -> Result<(), Box<dyn Error>> {
    let link = file_utils::symlink_path(runtime);
    #[cfg(target_os = "windows")]
    let output = run_command(
        "powershell",
        &["-Command", &format!("rm -r {}", link.display())],
    );
    #[cfg(unix)]
    let output = run_command("rm", &["-rf", link.to_str().unwrap_or_default()]);

    if matches!(output, Ok(output) if output.status.success()) {
        println!("deactivation successful");
    } else {
        println!("deactivation failed");
    }
    Ok(())
}
