use clap::{Parser, Subcommand};

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
    Gc { },
    Home {
        machine: Option<String>,
    },
    Rebuild { },
    Repl { },
    Update { },
    Diff { },
}

pub fn parse_args() -> Arguments {
    Arguments::parse()
}
