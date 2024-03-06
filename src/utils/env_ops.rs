use crate::constants::versions::constants;

pub fn read_versions() -> Result<(), Box<dyn std::error::Error>> {
    let available_versions = constants::AVAILABLE_VERSIONS;
    let versions: Vec<&str> = available_versions.split(',').collect();

    println!("Availaible Versions:");
    for version in versions {
        println!("{}", version);
    }

    Ok(())
}

pub fn get_download_link(name: String, os: &str) -> Result<String, Box<dyn std::error::Error>> {
    let env_var_name = format!("{}_{}", name.to_uppercase(), os.to_uppercase());
    if let Some(value) = constants::get_constant(&env_var_name) {
        Ok(value.to_string())
    } else {
        Err("Couldn't recognize OS or the specified JDK is not available".into())
    }
}
