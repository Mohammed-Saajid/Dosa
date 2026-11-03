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
    #[command(name = "delete", about = "Delete Project")]
    Delete { name: String },
    #[command(name = "todo", about = "Manage global todos")]
    Todo {
        #[command(subcommand)]
        command: TodoCommands,
    },
    #[command(name = "project", about = "Manage project todos")]
    Project {
        name: String,
        #[command(subcommand)]
        command: TodoCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum TodoCommands {
    #[command(about = "Add a todo")]
    Add { title: String },
    #[command(about = "List todos")]
    List,
    #[command(name = "complete", about = "Mark a todo as complete")]
    Complete { id: u64 },
    #[command(about = "Remove a todo")]
    Remove { id: u64 },
}
