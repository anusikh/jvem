use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Uninstall an installed JDK, e.g. `jvem java uninstall zulu17`.
pub fn uninstall(name: String) -> Result<(), Box<dyn Error>> {
    common::uninstall(Runtime::Java, &name)
}
