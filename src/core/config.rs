use anyhow::{Context,Result};
use std::{env,fs, path::PathBuf};

#[derive(Debug,serde::Deserialize)]
pub struct Config {
    pub default_code_editor:String,
    pub default_projects_dir:String,
}


pub fn read_config_file() -> Result<String> {
    let home = env::var("HOME")
        .context("Failed to Read Home Directory")?;

    let config_path = PathBuf::from(home).join(".dosaconfig");

    let content = fs::read_to_string(&config_path)
        .context("NO DOSA")?;
        
    Ok(content)

}


pub fn load_config(config:&str) -> Result<Config> {
    let config: Config = toml::from_str(config)?;
    Ok(config)
}
