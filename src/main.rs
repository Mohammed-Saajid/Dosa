use anyhow::{Context, Result};
use dialoguer::Select;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
mod core;
mod editors;
mod projects;
use crate::core::config::{Config, load_config, prompt, read_config_file};
use crate::editors::Editor;
use crate::projects::{Project,select_project,discover_projects};



fn main() -> Result<()> {
    let config_file = read_config_file();

    let config = match config_file {
        Ok(contents) => load_config(&contents)?,
        Err(_) => Config {
            default_code_editor: prompt("Default Code Editor")?,
            default_projects_dir: prompt("Default Project directory")?,
        },
    };

    let projects_dir = Path::new(&config.default_projects_dir);
    let projects: Vec<Project> = discover_projects(&projects_dir)?;

    if projects.is_empty() {
        println!("No projects found in {}", projects_dir.display());
        return Ok(());
    }

    let selection: usize = select_project(&projects)?;
    if let Some(editor) = Editor::from_str(&config.default_code_editor) {
        editor.launch(&projects[selection].path)?
    } else {
        panic!(
            "{}",
            format!(
                "Code Editor {} still not supported.",
                &config.default_code_editor
            )
        );
    }

    Ok(())
}

