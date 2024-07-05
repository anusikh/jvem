use std::{
    env, fs, io,
    path::Path,
    process::{Command, Output},
};

use lazy_static::lazy_static;
use rand::Rng;

fn get_home_env() -> String {
    if env::consts::OS == "windows" {
        "USERPROFILE".to_string()
    } else {
        "HOME".to_string()
    }
}

lazy_static! {
    static ref HOME_DIR: String = env::var(get_home_env()).unwrap();
}

pub fn run_command(command: &str, args: Vec<&str>) -> Output {
    let output = Command::new(command).args(args).output().expect("failed");
    output
}

pub fn find_file_in_dir(base_path: &str, name: &str) -> String {
    let mut res: String = String::new();
    let entries = fs::read_dir(&base_path).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if file_name.to_str().unwrap().starts_with(&name)
                    && file_name.to_str().unwrap().ends_with(".gz")
                {
                    res = path.display().to_string()
                }
            }
        }
    }

    res
}

pub fn get_home_dir() -> String {
    let res = &HOME_DIR;
    res.to_string()
}

pub fn get_installation_dir(name: &str) -> String {
    format!("{}/.jvem/java_versions/{}", get_home_dir(), name)
}

pub fn check_jdk_exists(name: &str) -> bool {
    let d_path = get_installation_dir(name);
    match Path::new(&d_path).exists() {
        true => true,
        false => false,
    }
}

pub fn check_maven_exists() -> bool {
    let m_path = format!("{}/.jvem/maven/bin", get_home_dir());
    match Path::new(&m_path).exists() {
        true => true,
        false => false,
    }
}

pub fn check_path_exists(path: &str) -> bool {
    match Path::new(&path).exists() {
        true => true,
        false => false,
    }
}

pub fn create_java_dir(name: &str) {
    let new_dir_path = format!("{}/.jvem/java_versions/{}", *HOME_DIR, name);
    fs::create_dir_all(new_dir_path).unwrap();
}

pub fn create_maven_dir() {
    let new_dir = format!("{}/.jvem/maven", *HOME_DIR);
    fs::create_dir_all(new_dir).unwrap();
}

pub fn check_list_locally() {
    let jvem_dir = format!("{}/.jvem/java_versions/", get_home_dir());
    let path_dir = Path::new(&jvem_dir);
    match path_dir.exists() {
        true => {
            let mut res: Vec<String> = Vec::new();

            for entry in fs::read_dir(path_dir).unwrap() {
                let entry = entry.unwrap();
                // .DS_Store is macos specific
                if entry.file_name() != "java" && entry.file_name() != ".DS_Store" {
                    res.push(String::from(entry.file_name().to_str().unwrap()));
                }
            }
            if res.len() > 0 {
                for item in res {
                    println!("{}", item);
                }
            } else {
                println!("no jdk installations found locally");
            }
        }
        false => {
            println!("no jdk installations found locally");
        }
    }
}

pub fn is_empty_dir(path: &std::path::Path) -> io::Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}

pub fn clean_jvem() {
    for entry in fs::read_dir(format!("{}/.jvem/java_versions", get_home_dir())).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() && is_empty_dir(&path).unwrap() {
            fs::remove_dir(&path).unwrap();
        }
    }
}

pub fn extract_tarball_linux(name: String, command: String) {
    let tar_location = match command.as_str() {
        "java" => &find_file_in_dir("/tmp", &name),
        "maven" => "/tmp/maven.tar.gz",
        _ => "",
    };

    let ext_location = match command.as_str() {
        "java" => &format!("{}/.jvem/java_versions/{}", get_home_dir(), name),
        "maven" => &format!("{}/.jvem/maven", get_home_dir()),
        _ => "",
    };

    let tarball_status = run_command(
        "/usr/bin/tar",
        vec![
            "xvzf",
            tar_location,
            "--strip-components=1",
            "-C",
            ext_location,
        ],
    );

    if tarball_status.status.success() {
        println!("tarball extraction successful");
    } else {
        println!(
            "tarball extraction failed: {:?} ",
            String::from_utf8_lossy(&tarball_status.stderr)
        );
    }
}

pub fn extract_tarball_macos(name: &str, command: &str) {
    let temp_folder_name = format!("{}", rand::thread_rng().gen::<u32>());
    let tar_path = match command {
        "java" => &find_file_in_dir("/tmp/", &name),
        "maven" => "/tmp/maven.tar.gz",
        _ => "",
    };

    let output_path = match command {
        "java" => &format!("/tmp/{}", temp_folder_name),
        "maven" => &format!("{}/.jvem/maven", get_home_dir()),
        _ => "",
    };

    let res = fs::create_dir_all(format!("/tmp/{}", temp_folder_name));
    match res {
        Ok(_) => {
            let tarball_status = run_command(
                "/usr/bin/tar",
                vec!["xvzf", tar_path, "--strip-components=1", "-C", output_path],
            );

            if tarball_status.status.success() {
                println!("tarball extraction successful, trying to move some files around...");
                if command.eq("java") {
                    let mv_status = run_command(
                        "sh",
                        vec![
                            "-c",
                            &format!(
                            "mv $(find /tmp/{} -mindepth 1 -maxdepth 1 -type d | head -n 1)/* {}",
                            temp_folder_name,
                            &format!("{}/.jvem/java_versions/{}", get_home_dir(), name)
                        ),
                        ],
                    );

                    if mv_status.status.success() {
                        println!("done moving files around...");
                    } else {
                        println!(
                            "moving files failed: {:?} ",
                            String::from_utf8_lossy(&mv_status.stderr)
                        );
                    }
                }
            } else {
                println!(
                    "tarball extraction failed: {:?} ",
                    String::from_utf8_lossy(&tarball_status.stderr)
                );
            }
        }
        Err(_) => {
            println!("could not create a temporary directory");
        }
    }
}

pub fn extract_zip(temp_dir: &str, name: &str, command: String) {
    let output_path = match command.as_str() {
        "java" => &get_installation_dir(name),
        "maven" => &format!("{}/.jvem/maven", get_home_dir()),
        _ => "",
    };

    let unzip_output = run_command(
        "powershell",
        vec![
            "-Command",
            &format!(
                "$ProgressPreference = 'SilentlyContinue';Expand-Archive -Path {0} -DestinationPath {1}; mv {1}\\*\\* {1};$ProgressPreference = 'Continue'",
                temp_dir, output_path,
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
}
