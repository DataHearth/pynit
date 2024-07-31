use std::{fs::write, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use init::ask_user_inputs;
use toml::toml;

mod init;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Initialize a new python directory with minimal fields for "pyproject.toml"
    #[arg(short, long, default_value_t = true)]
    minimum: bool,

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
        Subcommands::New { folder } => todo!(),
        Subcommands::Init {} => {
            let pyproject = ask_user_inputs(args.minimum)?;
            write("pyproject.toml", toml::to_vec(&pyproject)?)?;
        }
    };

    Ok(())
}
