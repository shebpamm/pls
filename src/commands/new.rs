use color_eyre::eyre::Result;

use crate::cli::Templates;
use askama::Template;

#[derive(Template)]
#[template(path = "aspect.nix", escape = "none")]
struct AspectTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "wrapper.nix", escape = "none")]
struct WrapperTemplate {
    name: String,
}

enum RenderableTemplate {
    Aspect(AspectTemplate),
    Wrapper(WrapperTemplate),
}

impl RenderableTemplate {
    fn render(&self) -> Result<String> {
        match self {
            RenderableTemplate::Aspect(t) => Ok(t.render()?),
            RenderableTemplate::Wrapper(t) => Ok(t.render()?),
        }
    }
}

impl From<&Templates> for RenderableTemplate {
    fn from(template: &Templates) -> Self {
        let name = template.args().resolved_name();
        match template {
            Templates::Aspect { args: _ } => RenderableTemplate::Aspect(AspectTemplate { name }),
            Templates::Wrapper { args: _ } => RenderableTemplate::Wrapper(WrapperTemplate { name }),
        }
    }
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
    let t = RenderableTemplate::from(&template);
    let output = t.render()?;

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
