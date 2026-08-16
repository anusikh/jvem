use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::runtime::Runtime;
use crate::utils::file_utils::{home_dir, install_dir, is_empty_dir};

static DIR_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Create a unique, empty directory under the system temp dir so tests can
/// run in parallel without colliding.
fn unique_temp_dir(label: &str) -> PathBuf {
    let n = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("jvem-test-{label}-{}-{n}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn test_is_not_empty_dir() {
    let dir = unique_temp_dir("not-empty");
    fs::File::create(dir.join("sample.txt")).unwrap();
    assert!(!is_empty_dir(&dir).unwrap());
}

#[test]
fn test_is_empty_dir() {
    let dir = unique_temp_dir("empty");
    assert!(is_empty_dir(&dir).unwrap());
}

#[test]
fn test_install_dirs_live_under_jvem() {
    let home = home_dir();
    assert_eq!(
        install_dir(Runtime::Java, "zulu17"),
        home.join(".jvem").join("java_versions").join("zulu17")
    );
    assert_eq!(
        install_dir(Runtime::Node, "22.11.0"),
        home.join(".jvem").join("node_versions").join("22.11.0")
    );
    assert_eq!(
        install_dir(Runtime::Maven, "anything"),
        home.join(".jvem").join("maven")
    );
}
