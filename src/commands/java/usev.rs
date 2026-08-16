use std::error::Error;

use crate::commands::common;
use crate::runtime::Runtime;

/// Activate an installed JDK, e.g. `jvem java usev zulu17`.
pub async fn usev(name: String) -> Result<(), Box<dyn Error>> {
    common::use_version(Runtime::Java, &name).await
}
