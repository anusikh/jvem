//! Download link resolution and remote version listing.

use std::{env, error::Error};

use crate::constants::java_versions::{self, Jdk};
use crate::utils::file_utils::run_command;

/// Print all JDK versions available for install.
pub fn list_available_java() -> Result<(), Box<dyn Error>> {
    println!("available versions:");
    for jdk in java_versions::JDKS {
        println!("{}", jdk.name.to_ascii_lowercase());
    }
    Ok(())
}

/// Resolve the download URL for a JDK on the current platform.
pub fn java_download_link(name: &str) -> Result<String, Box<dyn Error>> {
    let jdk = java_versions::find(name).ok_or_else(|| {
        format!("unknown jdk '{name}'. run `jvem java lsrem` to see the available versions")
    })?;
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    jdk_url_for(jdk, os, arch)
        .map(str::to_string)
        .ok_or_else(|| format!("{} is not available for {os}/{arch}", jdk.name).into())
}

/// Pick the download URL for `jdk` on the given platform. Pure so it can be
/// unit tested independently of the build host.
fn jdk_url_for(jdk: &Jdk, os: &str, arch: &str) -> Option<&'static str> {
    match (os, arch) {
        ("windows", _) => Some(jdk.windows),
        ("linux", "x86_64") => Some(jdk.linux_x64),
        ("linux", "aarch64") => jdk.linux_aarch64,
        ("macos", "x86_64") => Some(jdk.macos_x64),
        ("macos", "aarch64") => jdk.macos_aarch64,
        _ => None,
    }
}

/// Resolve the download URL for a node version on the current platform.
pub fn node_download_link(version: &str) -> Result<String, Box<dyn Error>> {
    node_link_for(version, env::consts::OS, env::consts::ARCH).map_err(Into::into)
}

/// Build the nodejs.org download URL for `version` on the given platform.
/// Pure so it can be unit tested independently of the build host.
fn node_link_for(version: &str, os: &str, arch: &str) -> Result<String, String> {
    let (os_dir, extension) = match os {
        "windows" => ("win", "zip"),
        "macos" => ("darwin", "tar.gz"),
        "linux" => ("linux", "tar.gz"),
        other => return Err(format!("unsupported operating system: {other}")),
    };
    let arch_dir = match arch {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        other => return Err(format!("unsupported architecture: {other}")),
    };
    Ok(format!(
        "https://nodejs.org/dist/v{version}/node-v{version}-{os_dir}-{arch_dir}.{extension}"
    ))
}

/// Check that a node version exists on nodejs.org.
pub fn node_version_exists(version: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://nodejs.org/dist/v{version}/");
    // curl needs `NUL` on windows instead of `/dev/null`.
    #[cfg(target_os = "windows")]
    let output = run_command("curl", &["-s", "-o", "NUL", "-w", "%{http_code}", &url])?;
    #[cfg(unix)]
    let output = run_command(
        "curl",
        &["-s", "-o", "/dev/null", "-w", "%{http_code}", &url],
    )?;
    if !output.status.success() {
        return Err(format!(
            "couldn't connect to nodejs.org: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }
    match String::from_utf8_lossy(&output.stdout).trim() {
        "200" => Ok(()),
        "404" => Err(format!("no such node version exists: v{version}").into()),
        code => Err(format!("unexpected response from nodejs.org (HTTP {code})").into()),
    }
}

/// Print node versions available for install, grouped by major version.
/// Only major versions >= 16 are supported.
pub fn list_available_node() -> Result<(), Box<dyn Error>> {
    let output = run_command("curl", &["-fsSL", "https://nodejs.org/dist/"])?;
    if !output.status.success() {
        return Err(format!(
            "couldn't connect to nodejs.org: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }

    let body = String::from_utf8_lossy(&output.stdout);
    let mut groups: Vec<(u32, Vec<String>)> = Vec::new();
    for line in body.lines() {
        let Some(version) = parse_version(line) else {
            continue;
        };
        let Some(major) = major_version(version) else {
            continue;
        };
        if major < 16 {
            continue;
        }
        match groups.last_mut() {
            Some((current, versions)) if *current == major => {
                versions.push(version.to_string());
            }
            _ => groups.push((major, vec![version.to_string()])),
        }
    }

    if groups.is_empty() {
        println!("no supported node versions found");
        return Ok(());
    }
    println!("available versions:");
    for (major, versions) in groups {
        println!("nodejs v{major}: {}", versions.join(", "));
    }
    Ok(())
}

/// Extract the version from a nodejs.org dist listing line like
/// `<a href="v22.11.0/">22.11.0</a>`.
fn parse_version(line: &str) -> Option<&str> {
    const MARKER: &str = "href=\"v";
    let start = line.find(MARKER)? + MARKER.len();
    let rest = &line[start..];
    let end = rest.find('/')?;
    Some(&rest[..end])
}

/// The major component of a semver string like `"22.11.0"`.
fn major_version(version: &str) -> Option<u32> {
    version.split('.').next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{jdk_url_for, major_version, node_link_for, parse_version};
    use crate::constants::java_versions::find;

    #[test]
    fn parses_version_from_dist_listing() {
        assert_eq!(
            parse_version("<a href=\"v22.11.0/\">22.11.0</a>"),
            Some("22.11.0")
        );
        assert_eq!(parse_version("<a href=\"index.json\">index.json</a>"), None);
        assert_eq!(parse_version("no link here"), None);
    }

    #[test]
    fn parses_major_version() {
        assert_eq!(major_version("22.11.0"), Some(22));
        assert_eq!(major_version("v22"), None);
        assert_eq!(major_version(""), None);
    }

    #[test]
    fn builds_node_links_for_supported_platforms() {
        assert_eq!(
            node_link_for("22.11.0", "linux", "x86_64").unwrap(),
            "https://nodejs.org/dist/v22.11.0/node-v22.11.0-linux-x64.tar.gz"
        );
        assert_eq!(
            node_link_for("22.11.0", "macos", "aarch64").unwrap(),
            "https://nodejs.org/dist/v22.11.0/node-v22.11.0-darwin-arm64.tar.gz"
        );
        assert_eq!(
            node_link_for("22.11.0", "windows", "x86_64").unwrap(),
            "https://nodejs.org/dist/v22.11.0/node-v22.11.0-win-x64.zip"
        );
    }

    #[test]
    fn rejects_unsupported_platforms_for_node() {
        assert!(node_link_for("22.11.0", "freebsd", "x86_64").is_err());
        assert!(node_link_for("22.11.0", "linux", "sparc64").is_err());
    }

    #[test]
    fn resolves_jdk_urls_per_platform() {
        let zulu17 = find("zulu17").unwrap();
        assert_eq!(
            jdk_url_for(zulu17, "linux", "x86_64"),
            Some("https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_x64.tar.gz")
        );
        assert_eq!(
            jdk_url_for(zulu17, "windows", "x86_64"),
            Some("https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-win_x64.zip")
        );
        assert!(jdk_url_for(zulu17, "linux", "aarch64").is_some());
        assert_eq!(jdk_url_for(zulu17, "freebsd", "x86_64"), None);
    }

    #[test]
    fn openjdk11_is_not_available_on_aarch64_linux() {
        let openjdk11 = find("openjdk11").unwrap();
        assert_eq!(jdk_url_for(openjdk11, "linux", "aarch64"), None);
        assert!(jdk_url_for(openjdk11, "linux", "x86_64").is_some());
    }
}
