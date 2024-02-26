use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    linux: String,
    windows: String,
    macos: String,
}

pub fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("src/commands/sources.csv").unwrap();

    for record in reader.deserialize() {
        let record: Record = record?;
        println!("{}", record.name);
    }

    Ok(())
}

pub fn get_download_link(name: String, os: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("src/commands/sources.csv").unwrap();
    let mut res = "".to_string();
    for record in reader.deserialize() {
        let record: Record = record?;
        if name == record.name {
            if os == "linux" {
                res = record.linux;
            } else if os == "windows" {
                res = record.windows;
            } else if os == "macos" {
                res = record.macos;
            }
        }
    }

    if res == "".to_string() {
        Err("couldn't recogonize os or the specified jdk is not available".into())
    } else {
        Ok(res)
    }
}
