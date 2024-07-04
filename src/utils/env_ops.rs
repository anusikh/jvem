use crate::constants::java_versions::constants;

pub fn read_versions() -> Result<(), Box<dyn std::error::Error>> {
    let available_versions = constants::AVAILABLE_VERSIONS;
    let versions: Vec<&str> = available_versions.split(',').collect();

    println!("Availaible Versions:");
    for version in versions {
        println!("{}", version.to_ascii_lowercase());
    }

    Ok(())
}

pub fn get_download_link(
    name: String,
    os: &str,
    arch: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut env_var_name = format!("{}_{}", name.to_uppercase(), os.to_uppercase());
    if os == "macos" || os == "linux" {
        env_var_name = format!(
            "{}_{}_{}",
            name.to_uppercase(),
            os.to_uppercase(),
            arch.to_uppercase()
        );
    }
    if let Some(value) = constants::get_constant(&env_var_name) {
        Ok(value.to_string())
    } else {
        Err(
            "couldn't recognize OS, unsupported architecture or the specified JDK is not available"
                .into(),
        )
    }
}
