use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{cli::main_struct::TodoCommands, todos};

const GLOBAL_FILE: &str = ".dosa-todos.toml";
const PROJECT_FILE: &str = ".dosa-todos.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TodoStore {
    #[serde(default)]
    todos: Vec<Todo>,
}

pub fn add_global(title: &str) -> Result<()> {
    add(&global_path()?, title)
}

pub fn add_project(project_path: &Path, title: &str) -> Result<()> {
    add(&project_path.join(PROJECT_FILE), title)
}

pub fn list_global() -> Result<()> {
    list(&global_path()?)
}

pub fn list_project(project_path: &Path) -> Result<()> {
    list(&project_path.join(PROJECT_FILE))
}

pub fn complete_global(id: u64) -> Result<()> {
    complete(&global_path()?, id)
}

pub fn complete_project(project_path: &Path, id: u64) -> Result<()> {
    complete(&project_path.join(PROJECT_FILE), id)
}

pub fn remove_global(id: u64) -> Result<()> {
    remove(&global_path()?, id)
}

pub fn remove_project(project_path: &Path, id: u64) -> Result<()> {
    remove(&project_path.join(PROJECT_FILE), id)
}

fn global_path() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|path| path.join(GLOBAL_FILE))
        .context("Unable to find home directory")
}

fn add(path: &Path, title: &str) -> Result<()> {
    let title = title.trim();
    if title.is_empty() {
        bail!("Todo title cannot be empty");
    }
    let mut store = load(path)?;
    let id = store.todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    store.todos.push(Todo {
        id,
        title: title.to_owned(),
        completed: false,
    });
    save(path, &store)?;
    println!("Added todo #{id}: {title}");
    Ok(())
}

fn list(path: &Path) -> Result<()> {
    let store = load(path)?;
    if store.todos.is_empty() {
        println!("No todos found.");
        return Ok(());
    }
    for todo in store.todos {
        let marker = if todo.completed { "x" } else { " " };
        println!("#{:<3} [{}] {}", todo.id, marker, todo.title);
    }
    Ok(())
}

fn complete(path: &Path, id: u64) -> Result<()> {
    let mut store = load(path)?;
    let todo = store
        .todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .with_context(|| format!("Todo #{id} does not exist"))?;
    todo.completed = true;
    save(path, &store)?;
    println!("Completed todo #{id}");
    Ok(())
}

fn remove(path: &Path, id: u64) -> Result<()> {
    let mut store = load(path)?;
    let original_len = store.todos.len();
    store.todos.retain(|todo| todo.id != id);
    if store.todos.len() == original_len {
        bail!("Todo #{id} does not exist");
    }
    save(path, &store)?;
    println!("Removed todo #{id}");
    Ok(())
}

fn load(path: &Path) -> Result<TodoStore> {
    if !path.exists() {
        return Ok(TodoStore::default());
    }
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&content).with_context(|| format!("Invalid todo file: {}", path.display()))
}

fn save(path: &Path, store: &TodoStore) -> Result<()> {
    let content = toml::to_string_pretty(store).context("Failed to serialize todos")?;
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}



pub fn handle_todo(command: TodoCommands, project: Option<String>, projects_dir: &Path) -> Result<()> {
    let project_path = project.map(|name| projects_dir.join(name));
    if let Some(path) = &project_path {
        if !path.is_dir() {
            anyhow::bail!("Project directory does not exist: {}", path.display());
        }
    }
    match (command, project_path) {
        (TodoCommands::Add { title }, None) => todos::add_global(&title),
        (TodoCommands::List, None) => todos::list_global(),
        (TodoCommands::Complete { id }, None) => todos::complete_global(id),
        (TodoCommands::Remove { id }, None) => todos::remove_global(id),
        (TodoCommands::Add { title }, Some(path)) => todos::add_project(&path, &title),
        (TodoCommands::List, Some(path)) => todos::list_project(&path),
        (TodoCommands::Complete { id }, Some(path)) => todos::complete_project(&path, id),
        (TodoCommands::Remove { id }, Some(path)) => todos::remove_project(&path, id),
    }
}
