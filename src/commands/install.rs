use std::error::Error;

use crate::utils::csv_ops::get_download_link;

use crate::utils::file_utils::{
    check_jdk_exists, create_java_dir, extract_tarball, find_file_in_dir, get_home_dir,
    get_installation_dir, run_command,
};

#[cfg(target_os = "windows")]
fn install_util(name: String, link: String) {
    match check_jdk_exists(&name) {
        false => {
            create_java_dir(&name);

            let temp_directory = format!("{}/AppData/Local/Temp/{}.zip", get_home_dir(), name);
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

                let unzip_output = run_command(
                    "powershell",
                    vec![
                        "-Command",
                        &format!(
                            "Expand-Archive -Path {0} -DestinationPath {1}; mv {1}\\*\\* {1}",
                            &temp_directory,
                            get_installation_dir(&name),
                        ),
                    ],
                );

                if unzip_output.status.success() {
                    println!("unzipping successful ");
                } else {
                    println!(
                        "unzipping failed: {} ",
                        String::from_utf8_lossy(&unzip_output.stderr)
                    );
                }
            } else {
                println!(
                    "fetching zip failed: {} ",
                    String::from_utf8_lossy(&output.stderr)
                );
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
