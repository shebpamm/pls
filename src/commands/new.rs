use color_eyre::eyre::Result;

use crate::cli::{Templates, TemplateArgs};
use askama::Template;

#[derive(Template)]
#[template(path = "aspect.nix")]
struct AspectTemplate<'a> {
    args: &'a TemplateArgs,
}

pub fn new(ctx: crate::context::Context, template: Templates) -> Result<()> {
    match template {
        Templates::Aspect { args } => {
            let aspect = AspectTemplate { args: &args }; 
            println!("{}", aspect.render()?);
        }

        _ => todo!()
    }

    Ok(())
}
