use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "pls", version)]
pub struct Arguments {
    #[clap(flatten)]
    pub global: GlobalOptions,

    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Parser)]
pub struct GlobalOptions {
    #[clap(short, long, help = "Enable verbose output")]
    pub verbose: bool,
    #[clap(short, long, help = "Only print the the commands to-be executed")]
    pub only_print: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Check {
        target: Option<String>,
    },
    Gc {},
    Home {
        machine: Option<String>,
    },
    Rebuild {},
    Repl {},
    Update {},
    Diff {},
    New {
        #[clap(subcommand)]
        template: Templates,
    },
}

#[derive(Debug, Parser)]
pub struct TemplateArgs {
    #[clap(help = "Path to module")]
    pub path: PathBuf,
    #[clap(short, long, help = "Name of the module")]
    pub name: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Templates {
    Aspect {
        #[clap(flatten)]
        args: TemplateArgs,
    },
    Lib {
        #[clap(flatten)]
        args: TemplateArgs,
    },
}

pub fn parse_args() -> Arguments {
    Arguments::parse()
}
