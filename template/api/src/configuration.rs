use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub log_directive: String,
    pub front_base_url: String,
    pub api_path: String,
    pub static_path: Option<String>,
    pub static_dir: Option<String>,
}

impl Settings {
    pub fn new_from_file(file: Option<String>) -> Result<Self, ConfigError> {
        let config_file_required = file.is_some();
        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config".into());
        let config_file = file.unwrap_or_else(|| {
            env::var("CONFIG_FILE").unwrap_or_else(|_| format!("{}/dev", &config_path))
        });

        let default_config_file = format!("{}/default", config_path);
        let local_config_file = format!("{}/local", config_path);
        println!(
            "Trying to load {:?} config files",
            vec![&default_config_file, &local_config_file, &config_file]
        );

        let config = Config::builder()
            .add_source(File::with_name(&default_config_file))
            .add_source(File::with_name(&local_config_file).required(false))
            .add_source(File::with_name(&config_file).required(config_file_required))
            .add_source(Environment::with_prefix("{{crate_name}}"))
            .build()?;

        config.try_deserialize()
    }

    pub fn new() -> Result<Self, ConfigError> {
        Settings::new_from_file(None)
    }
}
