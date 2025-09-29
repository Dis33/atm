# Welcome to ATM

**ATM**, **A**gent **T**ooling **M**anagement utility
is orchestration tool for MCP servers,
fully written in Rust.

## Syntax

### Commands

- `sync`, `-S`  : Synchronize package from repository
  - **Options**:
    - `-y` : If package is already installed, Update it; Otherwise, option is ignored


- `remove`, `-R` : Remove packages


- `query`, `-Q` : Query packages
  - **Options**:
    - `-i` : Display information of package

### Options

- `-h`, `--help` : Print help
- `-V`, `--version` : Print version

## Sample Usage

- `atm --daemon` : Run ATM daemon
- `atm -S <git url>` : Install package from Git url
- `atm -Sy <git url>` : Install/Update package from Git url
- `atm -R <name>` : Uninstall package with given name
