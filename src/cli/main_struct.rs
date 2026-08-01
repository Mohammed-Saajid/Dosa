use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dosa", version, about = "Fast Project Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "new", about = "Create New Project")]
    New { name: String },
}
