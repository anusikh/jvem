use std::fs;

use crate::utils::file_utils::get_home_dir;

pub fn uninstall() {
    let path = format!("{}/.jvem/maven", get_home_dir());
    let res = fs::remove_dir_all(path);
    match res {
        Ok(_) => println!("maven uninstall successful"),
        Err(_) => println!("something went wrong uninstalling maven"),
    }
}
