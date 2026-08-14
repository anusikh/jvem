use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Deactivate the currently active node version.
pub fn deactivate() -> Result<(), Box<dyn Error>> {
    common::deactivate(Runtime::Node)
}
