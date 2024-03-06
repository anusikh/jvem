use std::error::Error;

use crate::utils::env_ops::read_versions;

pub fn lsrem() -> Result<(), Box<dyn Error>> {
    read_versions()?;
    Ok(())
}
