use anyhow::Result;
use std::path::Path;
mod core;
mod editors;
mod projects;
use crate::core::config::Config;
use crate::editors::Editor;
use crate::projects::{Project, discover_projects, select_project};

fn main() -> Result<()> {
    let config = Config::load_config()?;

    let projects_dir = Path::new(&config.projects_dir);
    let projects: Vec<Project> = discover_projects(&projects_dir)?;

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
        editor.launch(&projects[selection].path)?
    } else {
        panic!(
            "{}",
            format!("Code Editor {} still not supported.", &config.code_editor)
        );
    }

    Ok(())
}
