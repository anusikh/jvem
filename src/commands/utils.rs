use std::{
    env, fs,
    path::Path,
    process::{Command, Output},
};

use lazy_static::lazy_static;

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
    format!("{}\\.jvem\\{}", get_home_dir(), name)
}

pub fn check_jdk_exists(name: &str) -> bool {
    let dir_path = format!("{}/.jvem/{}", *HOME_DIR, name);
    match Path::new(&dir_path).exists() {
        true => true,
        false => false,
    }
}

pub fn create_java_dir(name: &str) {
    let new_dir_path = format!("{}/.jvem/{}", *HOME_DIR, name);
    fs::create_dir_all(new_dir_path).unwrap();
}
