use std::error::Error;

use crate::commands::{
    csv_ops::get_download_link,
    utils::{get_home_dir, run_linux_command},
};

use super::utils::{check_jdk_exists, create_java_dir, find_file_in_dir};

fn install_linux(name: String, link: String) {
    match check_jdk_exists(&name) {
        false => {
            create_java_dir(&name);

            let output =
                run_linux_command("/usr/bin/wget", vec![&format!("{}", link), "-P", "/tmp/"]);
            if output.status.success() {
                let tarball_status = run_linux_command(
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
                    print!("tarball extraction successful");
                } else {
                    println!(
                        "tarball extraction failed: {:?}",
                        String::from_utf8_lossy(&tarball_status.stderr)
                    );
                }
            } else {
                println!("fetching tarball failed");
            }
        }
        true => {
            println!("jdk already exists in fs");
        }
    }
}

pub fn install(name: String) {
    // TODO:
    // Check operating system x
    // Get the list of the available jdks x
    // Check if the name provided here matches any of these jdks x
    // Installation process: all 3 os'es
    // - wget the link x
    // - tar unzip it
    // store it in .jvem directory at base user
    println!(
        "triggered install with param {} {}",
        name,
        std::env::consts::OS
    );

    let res: Result<String, Box<dyn Error>> = get_download_link(name.clone(), std::env::consts::OS);
    match res {
        Ok(x) => install_linux(name, x),
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
}
