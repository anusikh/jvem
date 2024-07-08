use core::panic;

use crate::{constants::java_versions::constants, utils::file_utils::run_command};

pub fn read_versions() -> Result<(), Box<dyn std::error::Error>> {
    let available_versions = constants::AVAILABLE_VERSIONS;
    let versions: Vec<&str> = available_versions.split(',').collect();

    println!("Availaible Versions:");
    for version in versions {
        println!("{}", version.to_ascii_lowercase());
    }

    Ok(())
}

pub fn get_download_link(
    name: String,
    os: &str,
    arch: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut env_var_name = format!("{}_{}", name.to_uppercase(), os.to_uppercase());
    if os == "macos" || os == "linux" {
        env_var_name = format!(
            "{}_{}_{}",
            name.to_uppercase(),
            os.to_uppercase(),
            arch.to_uppercase()
        );
    }
    if let Some(value) = constants::get_constant(&env_var_name) {
        Ok(value.to_string())
    } else {
        Err(
            "couldn't recognize OS, unsupported architecture or the specified JDK is not available"
                .into(),
        )
    }
}

pub fn read_versions_node() -> Result<(), Box<dyn std::error::Error>> {
    let output = run_command("curl", vec!["https://nodejs.org/dist/"]);
    let mut curr = 16;
    let mut display_vec = Vec::new();
    if output.status.success() {
        let body = String::from_utf8_lossy(&output.stdout);
        for line in body.lines() {
            if line.contains(">v") && line.contains("/<") {
                if let Some(version) = parse_version(&line) {
                    let num_ver: u32 = version.split('.').next().unwrap().parse().unwrap();
                    // NOTE: We are supporting versions above 16
                    if num_ver >= 16 {
                        if !num_ver.eq(&curr) {
                            println!("nodejs v{}", curr.to_string());
                            println!("{:?}", display_vec);
                            curr = num_ver;
                            display_vec.clear();
                        }
                        display_vec.push(version);
                    }
                }
            }
        }
    } else {
        println!(
            "couldn't connect to nodejs.org: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

fn parse_version(line: &str) -> Option<&str> {
    let start = line.find(">v")? + 2;
    let end = line.find("/<")?;
    line.get(start..end)
}

pub fn get_download_link_node(version: &str, os: &str, arch: &str) -> Result<String, Box<dyn std::error::Error>> {
    let os_mapped = match os {
        "windows" => "win",
        "macos" => "darwin",
        "linux" => "linux",
        _ => panic!("unsupported operating system"),
    };

    let arch_mapped = match arch {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        _ => panic!("unsupported architecture"),
    };

    let format_mapped = match os {
        "linux" | "macos" => "tar.gz",
        "windows" => "zip",
        _ => panic!("unsupported operating system"),
    };

    let link = format!(
        "https://nodejs.org/dist/v{0}/node-v{0}-{1}-{2}.{3}",
        version, os_mapped, arch_mapped, format_mapped
    );

    Ok(link)
}
