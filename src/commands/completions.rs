use clap::CommandFactory;
use clap_complete::{Generator, generate};
use color_eyre::eyre::Result;
use std::io::stdout;

use crate::context::Context;

pub fn completions<G: Generator>(ctx: Context, generator: G) -> Result<()> {
    if ctx.verbose {
        println!("Generating completion script...");
    }

    let mut cmd = crate::cli::Arguments::command();
    let name = cmd.get_name().to_owned();

    generate(generator, &mut cmd, name, &mut stdout());

    Ok(())
}
