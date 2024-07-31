use std::{
    env,
    fs::{self, File},
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::Result;
use clap::Parser;
use pyproject::Pyproject;

mod pyproject;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum Layout {
    Src,
    Flat,
}

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output will be more verbose
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Initialize with minimal fields for "pyproject.toml"
    #[arg(short, long)]
    complete: bool,

    /// Initialize folder with a git repository
    #[arg(long)]
    git: bool,

    /// Initialize a new virtual environment with given name in initialized directory
    #[arg(long)]
    venv: Option<String>,

    /// Define a layout for your project (https://setuptools.pypa.io/en/latest/userguide/package_discovery.html)
    #[arg(long, value_enum)]
    layout: Option<Layout>,

    #[command(subcommand)]
    subcommands: Subcommands,
}

#[derive(clap::Subcommand)]
enum Subcommands {
    New { folder: PathBuf },
    Init {},
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcommands {
        Subcommands::New { folder } => {
            fs::create_dir(&folder)?;
            let folder = folder.canonicalize()?;

            initialize_folder(folder, args.complete, args.layout, args.venv, args.git)?;
        }
        Subcommands::Init {} => {
            let folder = env::current_dir()?;

            initialize_folder(folder, args.complete, args.layout, args.venv, args.git)?;
        }
    };

    Ok(())
}

fn initialize_folder(
    folder: PathBuf,
    complete: bool,
    layout: Option<Layout>,
    venv: Option<String>,
    git: bool,
) -> Result<()> {
    // todo: avoid clone and maybe find a better way
    let mut pypro = Pyproject::new(folder.clone(), complete);

    pypro.ask_inputs()?;
    let project_name = pypro.get_project_name();

    pypro.create_file()?;

    if let Some(layout) = layout {
        let layout_inner_path = match layout {
            Layout::Src => {
                let inner_path = folder.join(format!("src/{project_name}"));
                fs::create_dir_all(&inner_path)?;

                inner_path
            }
            Layout::Flat => {
                let inner_path = folder.join(project_name);
                fs::create_dir(&inner_path)?;

                inner_path
            }
        };

        File::create(layout_inner_path.join("__init__.py"))?;
    }
    if let Some(venv) = venv {
        Command::new("python3")
            .args(&["-m", "venv", &venv])
            .current_dir(&folder)
            .stdout(Stdio::null())
            .status()?;
    }
    if git {
        Command::new("git")
            .args(&["init"])
            .current_dir(&folder)
            .stdout(Stdio::null())
            .status()?;
        fs::write(
            folder.join(".gitignore"),
            include_bytes!("gitignore_python.txt"),
        )?;
    }

    Ok(())
}
