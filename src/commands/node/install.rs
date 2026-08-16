use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;
use crate::utils::env_ops;

/// Install a node version, e.g. `jvem node install 22.11.0`.
pub fn install(version: String) -> Result<(), Box<dyn Error>> {
    env_ops::node_version_exists(&version)?;
    let link = env_ops::node_download_link(&version)?;
    common::install(Runtime::Node, &version, &link)
}
