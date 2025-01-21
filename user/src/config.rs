use config::{Config, File, FileFormat};
use secrecy::SecretString;
use serde::Deserialize;
use std::{env, path::PathBuf};


#[derive(Deserialize)]
pub struct UserConfig {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub jwt: JwtSettings
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub name: String
}

#[derive(Deserialize)]
pub struct JwtSettings {
    pub secret: SecretString
}

impl UserConfig {
    pub fn get() -> Self {
        
        // If testing locally using cargo then uses "config.yaml" present in root dir of repo
        // else need to pass environment variable CONFIG_DIR for "config.yaml" file location
        let mut config_file_path = match option_env!("CARGO_WORKSPACE_DIR") {
            Some(s) => PathBuf::from(s),
            None => PathBuf::from(&env::var("CONFIG_DIR").expect("Failed to get CONFIG_PATH"))
        };

        config_file_path.push("config.yaml");

        let config = Config::builder()
            .add_source(File::new(config_file_path.to_str().unwrap(), FileFormat::Yaml))
            .build()
            .expect("Failed to build Config")
            .try_deserialize::<UserConfig>()
            .expect("Failed to deserialize to UserConfig");

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_config() {
        let config = UserConfig::get();
    }
}
