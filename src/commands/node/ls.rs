use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// List locally installed node versions.
pub fn ls() -> Result<(), Box<dyn Error>> {
    common::list_local(Runtime::Node)
}
