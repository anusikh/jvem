use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Remove empty node version directories.
pub fn clean() -> Result<(), Box<dyn Error>> {
    common::clean(Runtime::Node)
}
