use color_eyre::eyre::Result;

use crate::cli::{Templates};
use askama::Template;

#[derive(Template)]
#[template(path = "aspect.nix", escape = "none")]
struct AspectTemplate {
    name: String,
}

pub fn new(_ctx: crate::context::Context, template: Templates) -> Result<()> {
    match template {
        Templates::Aspect { args } => {
            let name = args.resolved_name();

            let aspect = AspectTemplate { name }; 
            println!("{}", aspect.render()?);
        }

        _ => todo!()
    }

    Ok(())
}
