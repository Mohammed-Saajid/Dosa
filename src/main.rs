use anyhow::Result;
use clap::Parser;
use std::path::Path;
mod cli;
mod core;
mod editors;
mod projects;
use crate::cli::main_struct::{Cli, Commands};
use crate::core::config::{Config};
use crate::projects::{
    create_project_dir, project_picker,
};

fn main() -> Result<()> {
    let args = Cli::parse();
    let config = Config::load_config()?;
    let projects_dir = Path::new(&config.projects_dir);

    match args.command {
        Some(Commands::New { name }) => {
            create_project_dir(projects_dir, &name)?;
        }
        None => {
            project_picker(projects_dir, &config)?;
        }
    }

    Ok(())
}
