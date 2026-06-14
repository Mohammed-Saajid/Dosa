use anyhow::{Context, Result};
use dialoguer::Select;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
mod core;
mod editors;
use crate::core::config::{Config, load_config, prompt, read_config_file};
use crate::editors::Editor;

#[derive(Debug, Clone)]
struct Project {
    name: String,
    path: PathBuf,
}

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

fn discover_projects(projects_dir: &Path) -> Result<Vec<Project>> {
    let home = dirs::home_dir().unwrap();
    let projects_dir = PathBuf::from(home).join(projects_dir);
    let mut projects = Vec::new();

    for entry in fs::read_dir(&projects_dir)
        .with_context(|| format!("Failed to read {}", projects_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();

            projects.push(Project { name, path });
        }
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(projects)
}

fn select_project(projects: &[Project]) -> Result<usize> {
    let project_names: Vec<&str> = projects
        .iter()
        .map(|project| project.name.as_str())
        .collect();

    let selection = Select::new()
        .with_prompt("Select a project")
        .items(&project_names)
        .default(0)
        .interact()?;

    Ok(selection)
}
