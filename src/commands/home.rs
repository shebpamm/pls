use crate::utils::default_machine_name;
use color_eyre::eyre::Result;
use std::process::Command;

pub fn home(ctx: crate::context::Context, machine: Option<String>) -> Result<()> {
    if ctx.verbose {
        println!("Building home configuration...");
    }

    let machine = match machine {
        Some(name) => name,
        None => default_machine_name()?,
    };

    if ctx.verbose {
        println!("Using machine name: {}", machine);
    }

    let target = format!(
        "{}#homeConfigurations.{}.activationPackage",
        ctx.dotfiles, machine
    );

    if ctx.verbose {
        println!("Flake target: {}", target);
    }

    let mut cmd = Command::new("nix");
    cmd.arg("build")
        .arg(&target)
        .arg("--no-link")
        .arg("--print-out-paths")
        .env("NIXPKGS_ALLOW_UNFREE", "1")
        .stderr(std::process::Stdio::inherit());

    if ctx.only_print {
        println!("Command to be executed: {:?}", &cmd);
        return Ok(());
    }

    let output = cmd.output()?;

    if !output.status.success() {
        return Err(eyre::eyre!(
            "Nix build failed with status: {}",
            output.status
        ));
    }

    let derivation = String::from_utf8(output.stdout)?
        .lines()
        .next()
        .ok_or_else(|| eyre::eyre!("No output from nix build"))?
        .to_string();

    if ctx.verbose {
        println!("Built derivation at path: {}", derivation);
        println!("Activating home configuration...");
    }

    let activated = Command::new(format!("{}/activate", derivation))
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !activated.success() {
        return Err(eyre::eyre!(
            "Activation script failed with status: {}",
            activated
        ));
    }

    Ok(())
}
