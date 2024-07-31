# pynit

[![License](https://img.shields.io/crates/l/pynit)](https://gitea.antoine-langlois.net/DataHearth/pynit/src/branch/main/LICENSE)
[![Version](https://img.shields.io/crates/v/pynit)](https://crates.io/crates/pynit)

---

`pynit` speeds up the process of creating a new python project. It can initialise the project with `git`,
a virtual environment (using the `venv` module) and creating a [basic folder structure](https://setuptools.pypa.io/en/latest/userguide/package_discovery.html) if wanted.

## Usage

```bash
Small CLI tool to initialize a python project

Usage: pynit [OPTIONS] <COMMAND>

Commands:
  new
  init
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose          Output will be more verbose
  -c, --complete         Initialize with minimal fields for "pyproject.toml"
      --git              Initialize folder with a git repository
      --venv <VENV>      Initialize a new virtual environment with given name in initialized directory
      --layout <LAYOUT>  Define a layout for your project (https://setuptools.pypa.io/en/latest/userguide/package_discovery.html) [possible values: src, flat]
  -h, --help             Print help information
  -V, --version          Print version information
```

2 subcommands are available: `new` and `init`. Each of them have the same flags (defined globally).

### Global flags

#### --verbose

Adding this flag will make the `STDOUT` more verbose.

NOTE: the flag doesn't do anything currently as logging is not yet implemented.

#### -c/--complete

`--complete` allows you to control how much questions will be asked during the initialisation of the `pyproject.toml`. By default, only required fields will be asked: `build-sytem` section, `project.name` and `project.version`.

#### --git

Initialise a git repository

#### --venv

Add a virtual environment to your python project with a given `name`.

#### --layout

Add a default popular folder structure to your project.
Two options are available: `flat` and `src`.

For more information, check out [this setuptools section](https://setuptools.pypa.io/en/latest/userguide/package_discovery.html).

### new

`new` acts like `cargo new`. It take one argument which is the project name. Project name will be the folder name and used as default when asking questions to create a basic `pyproject.toml`.

The directory must NOT exist before creating the project.

#### Examples

```bash
$ pynit --git --venv .env --layout flat new my_project
$ exa -a --tree --level=2 my_project
my_project
├── .env
│  ├── ...
├── .git
│  ├── ...
├── .gitignore
├── my_project
│  └── __init__.py
└── pyproject.toml
```

### init

Like `new`, `init` does the same thing. Except it'll initiase the current directory.

NOTE: if a `.gitignore` and/or a `pyproject.toml` is present, they'll be truncated.
