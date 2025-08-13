use clap::{Parser, Subcommand};
use std::{env, process::Command as ProcessCommand};

#[derive(Debug, Parser)]
#[clap(name = "bix")]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init,
}


fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            let cwd = env::current_dir()?;
            let dir_name = cwd.file_name().unwrap().to_string_lossy();
            let bin_name = format!("{}-node", dir_name);

            ProcessCommand::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg(&bin_name)
                .status()?;
        },
        Command::Propose => {

        }
    }  
}

