use anyhow::{Context, Result};
use dialoguer::Input;
use std::{fs, path::PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub code_editor: String,
    pub projects_dir: String,
}

impl Config {
    pub fn load_config() -> Result<Config> {
        if let Ok(value) = Config::read_config_file() {
            let config_struct: Config = toml::from_str(&value)?;
            Ok(config_struct)
        } else {
            Config::new_config()
        }
    }
    fn read_config_file() -> Result<String> {
        let home = match dirs::home_dir() {
            Some(value) => value,
            None => panic!("Unable to find Home directory"),
        };

        let config_path = PathBuf::from(home).join("dosaconfig.toml");

        let content = fs::read_to_string(&config_path).context("NO DOSA")?;

        Ok(content)
    }

    fn prompt(text: &str) -> Result<String> {
        Ok(Input::new().with_prompt(text).interact_text()?)
    }

    fn new_config() -> Result<Config> {
        let config_struct = Config {
            code_editor: Config::prompt("Enter your Default Text Editor")?,
            projects_dir: Config::prompt(
                "Enter your Projects  directory name in the Home directory",
            )?,
        };
        let config_toml = toml::to_string(&config_struct)?;
        Config::write_config(&config_toml);
        Ok(config_struct)
    }
    fn write_config(config_str: &str) {
        let config_path = dirs::home_dir().unwrap().join("dosaconfig.toml");
        let _ = fs::write(config_path, config_str);
    }
}
