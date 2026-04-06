mod cli;
mod commands;
mod context;

use cli::Commands;

fn main() {
    let args = cli::parse_args();

    let ctx = context::Context::from(&args);
    match args.subcommand {
        Commands::Home { machine } => commands::home::home(ctx, machine),
        Commands::New { template } => commands::new::new(ctx, template),
        _ => unimplemented!(),
    };
}
