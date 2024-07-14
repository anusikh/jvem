use crate::utils::env_ops::get_download_link_node;
use crate::utils::file_utils::{
    check_node_exists, create_node_dir, extract_tarball_macos, run_command,
};

#[cfg(target_os = "windows")]
fn install_util(version: String, link: String) {
    match check_node_exists(&version) {
        false => {
            println!("{}", link);
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
            println!("{}", link);
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
            create_node_dir();

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
    match link {
        Ok(l) => install_util(version, l),
        Err(e) => println!("{}", e.to_string()),
    }
}
