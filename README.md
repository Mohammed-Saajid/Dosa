# Dosa

A fast, lightweight CLI project manager written in Rust that helps developers quickly navigate and open their project directories.

## Overview

Dosa streamlines project management by providing a unified interface to:
- Discover all projects in your designated projects directory
- Quickly search and select projects using fuzzy matching
- Instantly open selected projects in your preferred code editor
- Create and delete projects with simple commands

Perfect for developers who manage multiple projects and want a frictionless workflow.

## Features

- Fast startup – Written in Rust for speed and efficiency
- Fuzzy search – Find projects quickly with intelligent filtering
- Simple project management – Create and delete projects effortlessly
- Multi-editor support – VS Code and NeoVim (As of now)
- Easy configuration – Simple TOML-based configuration file
- Persistent config – Configuration stored in your home directory
- Global and project-specific todo lists

## Installation

### Prerequisites

- Rust 1.70 or later

### From Cargo

```bash
cargo install dosa
```

### Building from Source

```bash
git clone https://github.com/yourusername/dosa.git
cd dosa
cargo build --release
```

The binary will be available at `target/release/dosa`.

## Quick Start

### First Time Setup

When you run `dosa` for the first time, it will prompt you to configure:

1. Default Code Editor – Enter `code` for VS Code or `nvim` for NeoVim
2. Projects Directory – Path to your projects folder (relative to home directory)

Configuration is saved to `~/dosaconfig.toml`.

### Basic Usage

#### Open a Project

```bash
dosa
```

Displays a fuzzy-searchable list of all projects. Start typing to filter and select a project. The selected project opens in your configured editor.

#### Create a New Project

```bash
dosa new <project-name>
```

Creates a new project directory in your projects folder.

#### Delete a Project

```bash
dosa delete <project-name>
```

Removes a project directory.

#### Manage Todos

Global todos are stored in `~/.dosa-todos.toml`:

```bash
dosa todo add "Review pull request"
dosa todo list
dosa todo complete 1
dosa todo remove 1
```

Project todos are stored in the project directory as `.dosa-todos.toml`:

```bash
dosa project my-project add "Update dependencies"
dosa project my-project list
dosa project my-project complete 1
dosa project my-project remove 1
```

## Configuration

Configuration is stored in `~/dosaconfig.toml`:

```toml
code_editor = "code"  # or "nvim"
projects_dir = "Projects"  # relative to home directory
```

### Supported Editors

- VS Code – Configure with `code`
- NeoVim – Configure with `nvim`

## How It Works

1. Project Discovery – Dosa scans your projects directory for subdirectories
2. Fuzzy Selection – Projects are presented in an interactive, searchable list
3. Editor Launch – Selected project opens in your configured editor
4. Consistent Workflow – Manage all projects from a single command


## Requirements

- Rust 1.70+
- VS Code or NeoVim (for project launching)

## License

This project is licensed under the Apache License 2.0 – see the LICENSE file for details.

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.
