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
        Commands::Update { target } => commands::update::update(ctx, target),
        Commands::Completions { generator } => commands::completions::completions(ctx, generator),
        Commands::Rebuild { machine, action } => commands::rebuild::rebuild(ctx, machine, action),
        _ => unimplemented!(),
    }
    .unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
}
