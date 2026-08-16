use std::error::Error;

use crate::utils::env_ops;

/// List JDK versions available for install.
pub fn lsrem() -> Result<(), Box<dyn Error>> {
    env_ops::list_available_java()
}
