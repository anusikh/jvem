use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

// archive.apache.org hosts every release permanently; dlcdn.apache.org only
// keeps the latest one and would 404 for pinned versions.
const MAVEN_URL: &str =
    "https://archive.apache.org/dist/maven/maven-3/3.9.8/binaries/apache-maven-3.9.8-bin.tar.gz";
const MAVEN_URL_WINDOWS: &str =
    "https://archive.apache.org/dist/maven/maven-3/3.9.8/binaries/apache-maven-3.9.8-bin.zip";

/// Install maven to the system.
pub fn install() -> Result<(), Box<dyn Error>> {
    let link = if cfg!(windows) {
        MAVEN_URL_WINDOWS
    } else {
        MAVEN_URL
    };
    common::install(Runtime::Maven, "maven", link)
}
