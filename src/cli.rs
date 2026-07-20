use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "pls", version)]
pub struct Arguments {
    #[clap(flatten)]
    pub global: GlobalOptions,

    #[clap(subcommand)]
    pub subcommand: Commands,
}

#[derive(Debug, Args)]
pub struct GlobalOptions {
    #[clap(short, long, help = "Enable verbose output")]
    pub verbose: bool,
    #[clap(short, long, help = "Only print the the commands to-be executed")]
    pub only_print: bool,
}

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum RebuildAction {
    #[default]
    Switch,
    Boot,
    Test,
    Build,
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
    Rebuild {
        machine: Option<String>,

        #[clap(short, long, value_enum, help = "Action to perform")]
        #[arg(default_value_t)]
        action: RebuildAction,
    },
    Repl {},
    Update {
        target: Option<String>,
    },
    Diff {},
    New {
        #[clap(subcommand)]
        template: Templates,
    },
    Completions {
        #[clap(value_enum)]
        generator: Shell,
    },
}

#[derive(Debug, Args)]
pub struct TemplateArgs {
    #[clap(help = "Path to module")]
    pub path: PathBuf,
    #[clap(short, long, help = "Name of the module")]
    pub name: Option<String>,

    #[arg(long)]
    pub only: Option<Vec<String>>,
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
#[command(disable_help_subcommand = true)]
pub enum Templates {
    Aspect {
        #[clap(flatten)]
        args: TemplateArgs,
    },
    Wrapper {
        #[clap(flatten)]
        args: TemplateArgs,
    },
}

impl Templates {
    pub fn args(&self) -> &TemplateArgs {
        match self {
            Templates::Aspect { args } => args,
            Templates::Wrapper { args } => args,
        }
    }
}

pub fn parse_args() -> Arguments {
    Arguments::parse()
}
