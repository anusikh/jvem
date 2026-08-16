use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Uninstall a node version, e.g. `jvem node uninstall 22.11.0`.
pub fn uninstall(version: String) -> Result<(), Box<dyn Error>> {
    common::uninstall(Runtime::Node, &version)
}
