use anyhow::{Context, Result};
use clap::Parser;
mod cli;
mod core;
mod editors;
mod projects;
mod todos;
use crate::todos::{handle_todo};
use crate::cli::main_struct::{Cli, Commands};
use crate::core::config::Config;
use crate::projects::{create_project_dir, delete_project, project_picker};

fn main() -> Result<()> {
    let args = Cli::parse();
    let config = Config::load_config()?;
    let projects_dir = dirs::home_dir()
        .context("Unable to find home directory")?
        .join(&config.projects_dir);

    match args.command {
        Some(Commands::New { name }) => {
            create_project_dir(&projects_dir, &name)?;
        }
        Some(Commands::Delete { name }) => {
            delete_project(&projects_dir, &name)?;
        }
        Some(Commands::Todo { command }) => handle_todo(command, None, &projects_dir)?,
        Some(Commands::Project { name, command }) => {
            handle_todo(command, Some(name), &projects_dir)?
        }
        None => {
            project_picker(&projects_dir, &config)?;
        }
    }

    Ok(())
}
