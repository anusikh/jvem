use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Uninstall maven from the system.
pub fn uninstall() -> Result<(), Box<dyn Error>> {
    common::uninstall(Runtime::Maven, "maven")
}
