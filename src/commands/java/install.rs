use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;
use crate::utils::env_ops;

/// Install a JDK by name, e.g. `jvem java install zulu17`.
pub fn install(name: String) -> Result<(), Box<dyn Error>> {
    let link = env_ops::java_download_link(&name)?;
    common::install(Runtime::Java, &name, &link)
}
