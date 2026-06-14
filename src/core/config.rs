use anyhow::{Context, Ok, Result};
use dialoguer::Input;
use std::{collections::HashMap, env, fs, io::stdin, path::PathBuf, vec};
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub default_code_editor: String,
    pub default_projects_dir: String,
}

pub fn read_config_file() -> Result<String> {
    let home = match dirs::home_dir() {
        Some(value) => value,
        None => panic!("Unable to find Home directory"),
    };

    let config_path = PathBuf::from(home).join("dosaconfig.toml");

    let content = fs::read_to_string(&config_path).context("NO DOSA")?;

    Ok(content)
}

pub fn load_config(config: &str) -> Result<Config> {
    let config: Config = toml::from_str(config)?;
    Ok(config)
}

pub fn prompt(text: &str) -> Result<String> {
    Ok(Input::new().with_prompt(text).interact_text()?)
}
