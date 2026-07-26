use anyhow::{Context, Ok, Result};
use std::{path::Path, process::Command};
pub enum Editor {
    VSCode,
    NeoVim,
}

impl Editor {
    pub fn launch(&self, project_path: &Path) -> Result<()> {
        match self {
            Editor::VSCode => open_vscode(project_path),
            Editor::NeoVim => open_neovim(project_path),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "code" => Some(Editor::VSCode),
            "nvim" => Some(Editor::NeoVim),
            _ => None,
        }
    }
}

fn open_vscode(project_path: &Path) -> Result<()> {
    Command::new("code")
        .arg(project_path)
        .spawn()
        .context("Failed to launch VS Code")?;

    Ok(())
}

fn open_neovim(project_path: &Path) -> Result<()> {
    Command::new("nvim")
        .arg(project_path)
        .spawn()
        .context("Failed to launch NeoVim")?;
    Ok(())
}
