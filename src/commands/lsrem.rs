use std::error::Error;

use crate::utils::csv_ops::read_versions;

pub fn lsrem() -> Result<(), Box<dyn Error>> {
    read_versions()?;
    Ok(())
}
