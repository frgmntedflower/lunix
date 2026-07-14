//! Discovery: walk ~/.lunix/modules/, read every lunix.toml, build the
//! list the menu renders from. Per docs/MODULE_CONTRACT.md — no
//! registry, no network, just a directory scan.

use crate::manifest::{Manifest, ResolvedModule};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn modules_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home).join(".lunix").join("modules")
}

pub fn discover() -> Result<Vec<ResolvedModule>> {
    let root = modules_dir();
    let mut found = Vec::new();

    if !root.exists() {
        // Not an error — a fresh install just has no modules yet.
        return Ok(found);
    }

    for entry in fs::read_dir(&root).with_context(|| format!("reading {root:?}"))? {
        let entry = entry?;
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }

        let manifest_path = dir.join("lunix.toml");
        if !manifest_path.exists() {
            // A directory with no manifest isn't a module. Skip quietly —
            // this is a scan, not a linter.
            continue;
        }

        let raw = fs::read_to_string(&manifest_path)
            .with_context(|| format!("reading {manifest_path:?}"))?;
        let manifest: Manifest = toml::from_str(&raw)
            .with_context(|| format!("parsing {manifest_path:?}"))?;

        found.push(ResolvedModule { manifest, dir });
    }

    found.sort_by(|a, b| a.manifest.module.name.cmp(&b.manifest.module.name));
    Ok(found)
}
