use std::error::Error;

use lazy_static::lazy_static;

use crate::commands::{
    csv_ops::get_download_link,
    utils::{get_home_dir, get_installation_dir, run_command},
};

use super::utils::{check_jdk_exists, create_java_dir, find_file_in_dir};

lazy_static! {
    static ref SYSTEM_OS: String = std::env::consts::OS.to_string();
}

fn install_windows(name: String, link: String) {
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
                println!("fetching zip successful \n");

                let unzip_output = run_command(
                    "powershell",
                    vec![
                        "-Command",
                        &format!(
                            "Expand-Archive -Path {} -DestinationPath {}; mv {}\\*\\* {}",
                            &temp_directory,
                            get_installation_dir(&name),
                            get_installation_dir(&name),
                            get_installation_dir(&name)
                        ),
                    ],
                );

                if unzip_output.status.success() {
                    println!("unzipping successful \n");
                } else {
                    println!(
                        "unzipping failed: {} \n",
                        String::from_utf8_lossy(&unzip_output.stderr)
                    );
                }
            } else {
                println!(
                    "fetching zip failed: {} \n",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        true => {
            println!("jdk already exists in fs \n");
        }
    }
}

fn install_linux(name: String, link: String) {
    match check_jdk_exists(&name) {
        false => {
            create_java_dir(&name);

            let output = run_command("/usr/bin/wget", vec![&format!("{}", link), "-P", "/tmp/"]);
            if output.status.success() {
                println!("fetching tarball successful \n");

                let tarball_status = run_command(
                    "/usr/bin/tar",
                    vec![
                        "xzf",
                        &find_file_in_dir("/tmp/", &name),
                        "--strip-components=1",
                        "-C",
                        &format!("{}/.jvem/{}", get_home_dir(), name),
                    ],
                );

                if tarball_status.status.success() {
                    println!("tarball extraction successful \n");
                } else {
                    println!(
                        "tarball extraction failed: {:?} \n",
                        String::from_utf8_lossy(&tarball_status.stderr)
                    );
                }
            } else {
                println!("fetching tarball failed \n");
            }
        }
        true => {
            println!("jdk already exists in fs \n");
        }
    }
}

pub fn install(name: String) {
    println!(
        "triggered install with param {} {} \n",
        name,
        std::env::consts::OS
    );

    let res: Result<String, Box<dyn Error>> = get_download_link(name.clone(), std::env::consts::OS);
    match res {
        Ok(x) => {
            if *SYSTEM_OS == "linux".to_string() {
                install_linux(name, x);
            } else if *SYSTEM_OS == "windows".to_string() {
                install_windows(name, x);
            }
        }
        Err(e) => {
            println!("{} \n", e.to_string());
        }
    }
}
