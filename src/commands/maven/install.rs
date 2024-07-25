use lazy_static::lazy_static;

use crate::utils::file_utils::{
    check_maven_exists, create_maven_dir, extract_tarball_linux, extract_tarball_macos,
    extract_zip, find_file_in_dir, get_home_dir, run_command,
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
    match check_maven_exists() {
        false => {
            create_maven_dir();
            let temp_directory = format!("{}/AppData/Local/Temp/maven.zip", get_home_dir());
            if std::path::Path::new(&temp_directory).exists() {
                println!("fetched maven zip from cache");
                extract_zip(&temp_directory, "", "maven");
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
                    extract_zip(&temp_directory, "", "maven");
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
                extract_tarball_linux("", "maven");
            } else {
                let download =
                    run_command("/usr/bin/wget", vec![&MAVEN_DOWN_URL, "-O", "/tmp/maven.tar.gz"]);

                if download.status.success() {
                    println!("maven download successful");
                    extract_tarball_linux("", "maven");
                } else {
                    println!("fetching maven failed");
                }
            }
        }
        true => println!("maven already exists, if it doesn't please run the uninstall command and try re-installing it"),
    }
}

#[cfg(target_os = "macos")]
fn install_util() {
    match check_maven_exists() {
        false => {
            create_maven_dir();
            let temp_directory = "/tmp/maven.tar.gz";
            if std::path::Path::new(temp_directory).exists() {
                println!("fetching tarball from cache successful");
                extract_tarball_macos("", "maven");
            } else {
                println!("fetching tarball");
                let output = run_command(
                    "/usr/bin/curl",
                    vec!["-o", temp_directory,&MAVEN_DOWN_URL]
                );

                if output.status.success() {
                    println!("fetching tarball successful");
                    extract_tarball_macos("", "maven");
                } else {
                    println!(
                        "fetching tarball failed: {} ",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        },
        true => println!("maven already exists, if it doesn't please run the uninstall command and try re-installing it"),
    }
}

pub fn install() {
    let _ = install_util();
}
