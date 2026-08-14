use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Activate an installed node version, e.g. `jvem node usev 22.11.0`.
pub async fn usev(version: String) -> Result<(), Box<dyn Error>> {
    common::use_version(Runtime::Node, &version).await
}
