use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use anyhow::{Context, Result};
use dialoguer::Select;

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

    let selection = Select::new()
        .with_prompt("Select a project")
        .items(&project_names)
        .default(0)
        .interact()?;

    Ok(selection)
}
