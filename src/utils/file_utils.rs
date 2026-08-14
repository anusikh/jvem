//! Filesystem and process helpers shared by all runtimes.
//!
//! All jvem state lives under `~/.jvem`:
//! - `~/.jvem/java_versions/<jdk>` — installed JDKs
//! - `~/.jvem/node_versions/<version>` — installed node versions
//! - `~/.jvem/maven` — the maven distribution
//! - `~/.jvem/java`, `~/.jvem/node` — symlinks/junctions to the active version

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::LazyLock,
};

use crate::runtime::Runtime;

/// The user's home directory, resolved once. `HOME` on unix, `USERPROFILE`
/// on windows.
static HOME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let key = if env::consts::OS == "windows" {
        "USERPROFILE"
    } else {
        "HOME"
    };
    env::var(key)
        .map(PathBuf::from)
        .unwrap_or_else(|_| panic!("environment variable {key} is not set"))
});

/// The user's home directory.
pub fn home_dir() -> &'static Path {
    &HOME_DIR
}

/// Path to `~/.jvem`.
pub fn jvem_dir() -> PathBuf {
    home_dir().join(".jvem")
}

/// Directory where a specific version of `runtime` is installed.
pub fn install_dir(runtime: Runtime, name: &str) -> PathBuf {
    match runtime {
        Runtime::Maven => jvem_dir().join("maven"),
        _ => jvem_dir().join(runtime.versions_dir()).join(name),
    }
}

/// Path of the symlink/junction pointing at the active version.
pub fn symlink_path(runtime: Runtime) -> PathBuf {
    jvem_dir().join(runtime.symlink_name())
}

/// Path of the on-disk archive for a runtime, used as a download cache.
fn archive_path(runtime: Runtime, name: &str) -> PathBuf {
    let cache_dir = if env::consts::OS == "windows" {
        HOME_DIR.join("AppData").join("Local").join("Temp")
    } else {
        PathBuf::from("/tmp")
    };
    let stem = if runtime == Runtime::Maven {
        "maven"
    } else {
        name
    };
    let extension = if env::consts::OS == "windows" {
        "zip"
    } else {
        "tar.gz"
    };
    cache_dir.join(format!("{stem}.{extension}"))
}

/// Whether `runtime` (pinned to `name` where applicable) is already installed.
pub fn is_installed(runtime: Runtime, name: &str) -> bool {
    match runtime {
        Runtime::Maven => jvem_dir().join("maven").join("bin").exists(),
        _ => install_dir(runtime, name).exists(),
    }
}

/// Create the directory a version will be installed into.
pub fn create_install_dir(runtime: Runtime, name: &str) -> io::Result<()> {
    fs::create_dir_all(install_dir(runtime, name))
}

/// List locally installed versions for `runtime`, skipping macos
/// `.DS_Store` entries.
pub fn list_installed(runtime: Runtime) -> io::Result<Vec<String>> {
    let dir = jvem_dir().join(runtime.versions_dir());
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut installed = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if entry.file_name() != ".DS_Store" {
            installed.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    Ok(installed)
}

/// Remove empty version directories left behind by interrupted installs.
pub fn clean_empty_dirs(runtime: Runtime) -> io::Result<()> {
    let dir = jvem_dir().join(runtime.versions_dir());
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && is_empty_dir(&path)? {
            fs::remove_dir(&path)?;
        }
    }
    Ok(())
}

/// Whether a directory contains no entries.
pub fn is_empty_dir(path: &Path) -> io::Result<bool> {
    Ok(fs::read_dir(path)?.next().is_none())
}

/// Run a program and capture its output. Returns an error if the program
/// could not be spawned; a non-zero exit code is *not* an error here, so
/// callers can inspect [`Output::status`].
pub fn run_command(program: &str, args: &[&str]) -> io::Result<Output> {
    Command::new(program).args(args).output()
}

/// Download `link` to the cache path for `runtime`/`name`, reusing a
/// previously downloaded archive when one exists.
pub fn download_archive(runtime: Runtime, name: &str, link: &str) -> io::Result<PathBuf> {
    let archive = archive_path(runtime, name);
    if archive.exists() {
        println!("using cached archive at {}", archive.display());
        return Ok(archive);
    }

    println!("downloading {}...", archive.display());
    #[cfg(target_os = "windows")]
    download_archive_windows(&archive, link)?;
    #[cfg(unix)]
    download_archive_unix(&archive, link)?;
    Ok(archive)
}

#[cfg(target_os = "windows")]
fn download_archive_windows(archive: &Path, link: &str) -> io::Result<()> {
    let dest = archive.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "archive path is not valid utf-8",
        )
    })?;
    let output = run_command(
        "powershell",
        &[
            "-Command",
            "Set-Variable ProgressPreference SilentlyContinue ;",
            "Invoke-WebRequest",
            "-outf",
            dest,
            "-Uri",
            link,
        ],
    )?;
    check_success(output, "download")
}

#[cfg(unix)]
fn download_archive_unix(archive: &Path, link: &str) -> io::Result<()> {
    let dest = archive.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "archive path is not valid utf-8",
        )
    })?;
    let output = run_command("curl", &["-fsSL", "-o", dest, link])?;
    check_success(output, "download")
}

/// Extract a downloaded archive for `runtime`/`name` into its install dir.
#[cfg(target_os = "windows")]
pub fn extract_archive(runtime: Runtime, name: &str, archive: &Path) -> io::Result<()> {
    extract_zip(runtime, name, archive)
}

/// Extract a downloaded archive for `runtime`/`name` into its install dir.
#[cfg(target_os = "linux")]
pub fn extract_archive(runtime: Runtime, name: &str, archive: &Path) -> io::Result<()> {
    let dest = install_dir(runtime, name);
    extract_tar_stripped(archive, &dest)
}

/// Extract a downloaded archive for `runtime`/`name` into its install dir.
///
/// macos JDK tarballs wrap the payload in a `.jdk` bundle (and sometimes an
/// extra directory), so the payload has to be relocated after extraction;
/// node tarballs extract cleanly but are still moved to match the original
/// layout. Maven extracts directly into `~/.jvem/maven`.
#[cfg(target_os = "macos")]
pub fn extract_archive(runtime: Runtime, name: &str, archive: &Path) -> io::Result<()> {
    match runtime {
        Runtime::Maven => {
            let dest = install_dir(runtime, name);
            extract_tar_stripped(archive, &dest)
        }
        Runtime::Java => {
            let temp = unique_temp_dir();
            fs::create_dir_all(&temp)?;
            extract_tar_stripped(archive, &temp)?;

            let dest = install_dir(runtime, name);
            let command = format!(
                "mv $(find {} -mindepth 1 -maxdepth 1 -type d | head -n 1)/* {}",
                temp.display(),
                dest.display()
            );
            let output = run_command("sh", &["-c", &command])?;
            let _ = fs::remove_dir_all(&temp);

            check_success(output, "moving files")
        }
        Runtime::Node => {
            let temp = PathBuf::from("/tmp").join(name);
            fs::create_dir_all(&temp)?;
            extract_tar_stripped(archive, &temp)?;

            let dest = jvem_dir().join("node_versions");
            let command = format!("mv {} {}", temp.display(), dest.display());
            let output = run_command("sh", &["-c", &command])?;

            check_success(output, "moving files")
        }
    }
}

/// Extract a tarball with `tar xvzf --strip-components=1` into `dest`.
#[cfg(unix)]
fn extract_tar_stripped(archive: &Path, dest: &Path) -> io::Result<()> {
    let output = run_command(
        "/usr/bin/tar",
        &[
            "xvzf",
            archive.to_str().unwrap_or_default(),
            "--strip-components=1",
            "-C",
            dest.to_str().unwrap_or_default(),
        ],
    )?;
    check_success(output, "tarball extraction")
}

/// Expand a zip archive (windows) into the install dir of `runtime`/`name`.
#[cfg(target_os = "windows")]
fn extract_zip(runtime: Runtime, name: &str, archive: &Path) -> io::Result<()> {
    let dest = install_dir(runtime, name);
    let command = format!(
        "Set-Variable ProgressPreference = 'SilentlyContinue';Expand-Archive -Path {0} -DestinationPath {1}; mv {1}\\*\\* {1};",
        archive.display(),
        dest.display()
    );
    let output = run_command("powershell", &["-Command", &command])?;
    check_success(output, "unzipping")
}

/// Turn a successful [`Output`] into `()`, or an [`io::Error`] carrying the
/// program's stderr otherwise.
fn check_success(output: Output, action: &str) -> io::Result<()> {
    if output.status.success() {
        println!("{action} successful");
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "{action} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

/// A unique directory under `/tmp` used as a staging area during macos
/// extraction.
#[cfg(target_os = "macos")]
fn unique_temp_dir() -> PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    PathBuf::from("/tmp").join(format!("jvem-{}-{nanos}", std::process::id()))
}
