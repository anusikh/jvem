use crate::utils::env_ops::{check_valid_node_version, get_download_link_node};
use crate::utils::file_utils::{
    check_node_exists, create_node_dir, extract_tarball_linux, extract_tarball_macos, extract_zip,
    get_home_dir, run_command,
};

#[cfg(target_os = "windows")]
fn install_util(version: String, link: String) {
    match check_node_exists(&version) {
        false => {
            create_node_dir(&version);

            let temp_directory = format!("{}/AppData/Local/Temp/{}.zip", get_home_dir(), version);

            if std::path::Path::new(&temp_directory).exists() {
                println!("fetching tarball from cache successful");
                extract_zip(&temp_directory, &version, "node");
            } else {
                println!("fetching zip...");

                let output = run_command(
                    "powershell",
                    vec![
                        "-Command",
                        "Set-Variable ProgressPreference SilentlyContinue ;",
                        "Invoke-WebRequest",
                        "-outf",
                        &temp_directory,
                        "-Uri",
                        &link,
                    ],
                );

                if output.status.success() {
                    println!("fetching zip successful ");
                    extract_zip(&temp_directory, &version, "node");
                } else {
                    println!(
                        "fetching zip failed: {} ",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        true => {
            println!("node version exists already, if it doesn't run the clean command")
        }
    }
}

#[cfg(target_os = "linux")]
fn install_util(version: String, link: String) {
    match check_node_exists(&version) {
        false => {
            create_node_dir(&version);

            let temp_directory = format!("/tmp/{}.tar.gz", version);

            if std::path::Path::new(&temp_directory).exists() {
                println!("fetching tarball from cache successful");
                extract_tarball_linux(&version, "node");
            } else {
                println!("fetching tarball...");
                let output = run_command("/usr/bin/curl", vec!["-o", &temp_directory, &link]);

                if output.status.success() {
                    println!("fetching tarball successful ");
                    extract_tarball_linux(&version, "node");
                } else {
                    println!(
                        "fetching tarball failed: {} ",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        true => {
            println!("node version exists already, if it doesn't run the clean command")
        }
    }
}

#[cfg(target_os = "macos")]
fn install_util(version: String, link: String) {
    match check_node_exists(&version) {
        false => {
            create_node_dir(&version);

            let temp_directory = format!("/tmp/{}.tar.gz", version);

            if std::path::Path::new(&temp_directory).exists() {
                println!("fetching tarball from cache successful");
                extract_tarball_macos(&version, "node");
            } else {
                println!("fetching tarball...");
                let output = run_command("/usr/bin/curl", vec!["-o", &temp_directory, &link]);

                if output.status.success() {
                    println!("fetching tarball successful ");
                    extract_tarball_macos(&version, "node");
                } else {
                    println!(
                        "fetching tarball failed: {} ",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        true => {
            println!("node version already exists in fs, if it doesn't run the clean command")
        }
    }
}

pub fn install(version: String) {
    let link = get_download_link_node(&version, std::env::consts::OS, std::env::consts::ARCH);
    match check_valid_node_version(&version) {
        Ok(_) => match link {
            Ok(l) => install_util(version, l),
            Err(e) => println!("{}", e.to_string()),
        },
        Err(e) => println!("{}", e.to_string()),
    }
}
