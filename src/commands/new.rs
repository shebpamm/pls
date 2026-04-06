use color_eyre::eyre::Result;

use crate::cli::Templates;
use askama::Template;

#[derive(Template)]
#[template(path = "aspect.nix", escape = "none")]
struct AspectTemplate {
    name: String,
}

fn write(path: &std::path::Path, content: &str) -> Result<()> {
    // ensure the parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write content to file
    std::fs::write(path, content)?;

    Ok(())
}

pub fn new(ctx: crate::context::Context, template: Templates) -> Result<()> {
    let output = match &template {
        Templates::Aspect { args } => {
            let name = args.resolved_name();

            let aspect = AspectTemplate { name };

            aspect.render()?
        }

        _ => todo!(),
    };

    if ctx.only_print {
        println!(
            "would write to {}:\n\n{}",
            template.args().path.display(),
            output
        );
    } else {
        write(&template.args().path, &output)?;
        println!("Created file at {}", template.args().path.display());
    }

    Ok(())
}
