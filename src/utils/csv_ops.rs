use std::error::Error;

pub fn read_versions() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    if let Ok(available_versions) = std::env::var("AVAILABLE_VERSIONS") {
        let versions: Vec<&str> = available_versions.split(',').collect();

        for version in versions {
            println!("{}", version);
        }
    }

    Ok(())
}

pub fn get_download_link(name: String, os: &str) -> Result<String, Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Access the environment variable for the specified name
    let env_var_name = format!("{}_{}", name, os.to_uppercase());
    match std::env::var(&env_var_name) {
        Ok(value) => Ok(value),
        Err(_) => Err("Couldn't recognize OS or the specified JDK is not available".into()),
    }
}