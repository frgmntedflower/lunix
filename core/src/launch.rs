//! Launching a module. v0.1 only implements `mode = "foreground"` —
//! that covers everything we actually need right now. `detached` is
//! defined in the contract for later and intentionally left
//! unimplemented here rather than half-implemented.

use crate::manifest::ResolvedModule;
use anyhow::{bail, Context, Result};
use std::process::Command;

pub enum LaunchOutcome {
    Clean,
    Failed(i32),
}

pub fn launch(module: &ResolvedModule) -> Result<LaunchOutcome> {
    match module.manifest.launch.mode.as_str() {
        "foreground" => launch_foreground(module),
        "detached" => bail!(
            "module '{}' requests mode = detached, which is defined in \
             the contract but not yet implemented in lunix-core",
            module.manifest.module.name
        ),
        other => bail!(
            "module '{}' has unknown launch mode '{other}'",
            module.manifest.module.name
        ),
    }
}

fn launch_foreground(module: &ResolvedModule) -> Result<LaunchOutcome> {
    let binary = module.binary_path();

    let status = Command::new(&binary)
        .args(&module.manifest.launch.args)
        .current_dir(&module.dir)
        .status()
        .with_context(|| format!("failed to exec {binary:?}"))?;

    // Per docs/MODULE_CONTRACT.md: exit 0 is clean, anything else is
    // surfaced to the user rather than silently swallowed.
    match status.code() {
        Some(0) => Ok(LaunchOutcome::Clean),
        Some(code) => Ok(LaunchOutcome::Failed(code)),
        None => Ok(LaunchOutcome::Failed(-1)), // killed by signal
    }
}
