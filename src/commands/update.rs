use crate::context::Context;
use color_eyre::eyre::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Input {
    group: String,
}

#[derive(Debug, Deserialize)]
struct Inputs {
    #[serde(flatten)]
    inputs: HashMap<String, Input>,
}

fn do_update(ctx: Context, inputs: Option<Vec<String>>) -> Result<()> {
    if ctx.verbose {
        println!("Updating all targets...");
    }

    let mut cmd = Command::new("nix");
    cmd.arg("flake")
        .arg("update")
        .arg("--flake")
        .arg(&ctx.dotfiles)
        .stderr(std::process::Stdio::inherit());

    if let Some(inputs) = inputs {
        for input in inputs {
            cmd.arg(input);
        }
    }

    if ctx.only_print {
        println!("Command to be executed: {:?}", &cmd);
        return Ok(());
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(eyre::eyre!(
            "Nix flake update failed with status: {}",
            status
        ));
    }

    Ok(())
}

fn fetch_inputs(ctx: &Context) -> Result<Inputs> {
    let mut cmd = Command::new("nix");
    cmd.arg("run").arg(format!("{}#list-inputs", ctx.dotfiles));

    if ctx.verbose {
        println!("Fetching inputs with command: {:?}", &cmd);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(eyre::eyre!(
            "Failed to fetch inputs: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let inputs: Inputs = serde_json::from_slice(&output.stdout)?;
    Ok(inputs)
}

fn update_target(ctx: Context, target: String) -> Result<()> {
    let inputs = fetch_inputs(&ctx)?;

    if inputs.inputs.contains_key(&target) {
        return do_update(ctx, Some(vec![target]));
    };

    let group_inputs = inputs
        .inputs
        .iter()
        .filter(|(_, input)| input.group == target)
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();

    if group_inputs.is_empty() {
        return Err(eyre::eyre!(
            "No input or group found for target: {}",
            target
        ));
    }

    do_update(ctx, Some(group_inputs))
}

pub fn update(ctx: Context, target: Option<String>) -> Result<()> {
    match target {
        Some(target) => update_target(ctx, target),
        None => do_update(ctx, None),
    }
}
