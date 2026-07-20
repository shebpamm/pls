use color_eyre::eyre::Result;
use std::process::Command;

pub fn rebuild(
    ctx: crate::context::Context,
    machine: Option<String>,
    action: crate::cli::RebuildAction,
    target: Option<String>,
) -> Result<()> {
    if ctx.verbose {
        println!("Building home configuration...");
    }

    let mut cmd = Command::new("nixos-rebuild");

    cmd.arg(action.to_string())
        .env("NIXPKGS_ALLOW_UNFREE", "1")
        .stderr(std::process::Stdio::inherit());

    match machine {
        Some(machine) => {
            let target = target.unwrap_or_else(|| format!("root@{}.sorsa.cloud", machine));

            cmd.arg("--flake")
                .arg(format!("{}#{}", ctx.dotfiles, machine))
                .arg("--target-host")
                .arg(target);
        }
        None => {
            cmd.arg("--flake")
                .arg(format!("{}#", ctx.dotfiles))
                .arg("--sudo");
        }
    }

    if ctx.only_print {
        println!("Command to be executed: {:?}", &cmd);
        return Ok(());
    }

    let output = cmd.output()?;

    if !output.status.success() {
        return Err(eyre::eyre!("Rebuild failed with status: {}", output.status));
    }

    Ok(())
}
