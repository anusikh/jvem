use std::error::Error;

use crate::utils::csv_ops::get_download_link;

use crate::utils::file_utils::{
    check_jdk_exists, create_java_dir, extract_tarball, extract_zip, find_file_in_dir,
    get_home_dir, get_installation_dir, run_command,
};

#[cfg(target_os = "windows")]
fn install_util(name: String, link: String) {
    match check_jdk_exists(&name) {
        false => {
            create_java_dir(&name);

            let temp_directory = format!("{}/AppData/Local/Temp/{}.zip", get_home_dir(), name);

            if std::path::Path::new(&temp_directory).exists() {
                println!("fetching tarball from cache successful");
                extract_zip(&temp_directory, &name);
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
                        &format!("{}", link),
                    ],
                );

                if output.status.success() {
                    println!("fetching zip successful ");
                    extract_zip(&temp_directory, &name);
                } else {
                    println!(
                        "fetching zip failed: {} ",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        true => {
            println!("jdk already exists in fs, if it doesn't run the clean command");
        }
    }
}

#[cfg(target_os = "linux")]
fn install_util(name: String, link: String) {
    match check_jdk_exists(&name) {
        false => {
            let x = find_file_in_dir("/tmp/", &name);

            create_java_dir(&name);

            if x.ends_with(".gz") {
                println!("fetching tarball from cache successful");
                extract_tarball(name);
            } else {
                let output =
                    run_command("/usr/bin/wget", vec![&format!("{}", link), "-P", "/tmp/"]);
                if output.status.success() {
                    println!("fetching tarball successful ");
                    extract_tarball(name);
                } else {
                    println!("fetching tarball failed ");
                }
            }
        }
        true => {
            println!("jdk already exists in fs, if it doesn't run the clean command");
        }
    }
}

pub fn install(name: String) {
    let res: Result<String, Box<dyn Error>> = get_download_link(name.clone(), std::env::consts::OS);
    match res {
        Ok(x) => {
            install_util(name, x);
        }
        Err(e) => {
            println!("{} ", e.to_string());
        }
    }
}
