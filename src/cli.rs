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

impl TemplateArgs {
    pub fn resolved_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            self.path
                .file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or_else(|| {
                    panic!(
                        "Could not determine name from path: {}",
                        self.path.display()
                    )
                })
                .to_string()
        })
    }
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

impl Templates {
    pub fn args(&self) -> &TemplateArgs {
        match self {
            Templates::Aspect { args } => args,
            Templates::Lib { args } => args,
        }
    }
}


pub fn parse_args() -> Arguments {
    Arguments::parse()
}
