use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Show the currently active node and npm versions.
pub async fn current() -> Result<(), Box<dyn Error>> {
    common::current(Runtime::Node).await
}
