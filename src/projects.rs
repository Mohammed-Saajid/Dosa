use anyhow::{Context, Result};
use dialoguer::{FuzzySelect, theme::SimpleTheme};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{core::config::Config, editors::Editor};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

pub fn discover_projects(projects_dir: &Path) -> Result<Vec<Project>> {
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

pub fn select_project(projects: &[Project]) -> Result<usize> {
    let project_names: Vec<&str> = projects
        .iter()
        .map(|project| project.name.as_str())
        .collect();

    let selection = FuzzySelect::with_theme(&SimpleTheme)
        .with_prompt("Type to Filter Projects")
        .default(0)
        .items(&project_names)
        .interact_opt()
        .context("Failed to read Selection Prompt")?;

    match selection {
        Some(index) => Ok(index),
        None => anyhow::bail!("Selection Cancelled by user"),
    }
}

pub fn create_project_dir(projects_dir: &Path, name: &str) -> Result<()> {
    let project_path = projects_dir.join(name);
    if project_path.exists() {
        anyhow::bail!(
            "Project Directory already exists: {}",
            project_path.display()
        );
    }
    fs::create_dir_all(&project_path)?;
    println!("Project Directory created at : {}", project_path.display());
    Ok(())
}

pub fn project_picker(projects_dir: &Path, config: &Config) -> Result<()> {
    let projects: Vec<Project> = discover_projects(projects_dir)?;

    if projects.is_empty() {
        println!("No projects found in {}", projects_dir.display());
        return Ok(());
    }

    let selection: usize = match select_project(&projects) {
        Ok(index) => index,
        Err(_) => {
            println!("No Project Selected");
            return Ok(());
        }
    };
    if let Some(editor) = Editor::from_str(&config.code_editor) {
        editor.launch(&projects[selection].path)
    } else {
        panic!(
            "{}",
            format!("Code Editor {} still not supported.", &config.code_editor)
        );
    }?;
    Ok(())
}
