mod cli;
mod commands;
mod context;

use cli::Commands;

fn main() {
    let args = cli::parse_args();

    let ctx = context::Context::from(&args);
    match args.subcommand {
        Commands::Home { machine } => commands::home::home(ctx, machine),
        _ => unimplemented!(),
    };
}
