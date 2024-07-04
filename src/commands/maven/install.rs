use lazy_static::lazy_static;

use crate::utils::file_utils::{
    check_maven_exists, extract_tarball_linux, find_file_in_dir, get_home_dir, run_command,
    create_maven_dir
};

lazy_static! {
    static ref MAVEN_DOWN_URL: String = String::from(
        "https://dlcdn.apache.org/maven/maven-3/3.9.8/binaries/apache-maven-3.9.8-bin.tar.gz"
    );
    static ref MAVEN_DOWN_URL_WIN: String = String::from(
        "https://dlcdn.apache.org/maven/maven-3/3.9.8/binaries/apache-maven-3.9.8-bin.zip"
    );
}

#[cfg(target_os = "windows")]
fn install_util() {
    use crate::utils::file_utils::extract_zip;

    match check_maven_exists() {
        false => {
            create_maven_dir();
            let temp_directory = format!("{}/AppData/Local/Temp/maven.zip", get_home_dir());
            if std::path::Path::new(&temp_directory).exists() {
                println!("fetched maven zip from cache");
                extract_zip(&temp_directory, "", String::from("maven"));
            }else {
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
                        &MAVEN_DOWN_URL_WIN 
                    ],
                );

                if output.status.success() {
                    println!("fetching zip successful ");
                    extract_zip(&temp_directory, "", String::from("maven"));
                } else {
                    println!(
                        "fetching zip failed: {} ",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        },
        true => println!("maven already exists, if it doesn't please run the uninstall command and try re-installing it"),
    }
}

#[cfg(target_os = "linux")]
fn install_util() {
    match check_maven_exists() {
        false => {
            create_maven_dir();
            let x = find_file_in_dir("/tmp/", "maven");

            if x.ends_with(".gz") {
                println!("fetching maven from cache successful");
                extract_tarball_linux(String::new(), String::from("maven"));
            } else {
                let download =
                    run_command("/usr/bin/wget", vec![&MAVEN_DOWN_URL, "-O", "/tmp/maven.tar.gz"]);

                if download.status.success() {
                    println!("maven download successful");
                    extract_tarball_linux(String::new(), String::from("maven"));
                } else {
                    println!("fetching maven failed");
                }
            }
        }
        true => println!("maven already exists, if it doesn't please run the uninstall command and try re-installing it"),
    }
}

pub fn install() {
    let _ = install_util();
}
