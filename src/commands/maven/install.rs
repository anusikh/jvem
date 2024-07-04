use lazy_static::lazy_static;

use crate::utils::file_utils::{check_maven_exists, find_file_in_dir, run_command};

lazy_static! {
    static ref MAVEN_DOWN_URL: String = String::from(
        "https://dlcdn.apache.org/maven/maven-3/3.9.8/binaries/apache-maven-3.9.8-bin.tar.gz"
    );
}

#[cfg(target_os = "linux")]
fn install_util() {
    use crate::utils::file_utils::{extract_tarball_linux, get_home_dir};

    match check_maven_exists() {
        false => {
            let maven_path = format!("{}/.jvem/maven", get_home_dir());
            std::fs::create_dir_all(maven_path).unwrap();
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
