//! Types matching docs/MODULE_CONTRACT.md v0.1.
//! If you change the contract, change it here first — this struct
//! definition IS the contract, in enforceable form.

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub module: ModuleInfo,
    #[serde(default)]
    pub launch: LaunchInfo,
}

#[derive(Debug, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub binary: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default = "default_category")]
    pub category: String,
}

fn default_category() -> String {
    "misc".to_string()
}

#[derive(Debug, Deserialize, Default)]
pub struct LaunchInfo {
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default)]
    pub args: Vec<String>,
}

fn default_mode() -> String {
    "foreground".to_string()
}

/// A manifest resolved against the directory it was found in —
/// this is what the rest of lunix-core actually works with.
#[derive(Debug)]
pub struct ResolvedModule {
    pub manifest: Manifest,
    pub dir: PathBuf,
}

impl ResolvedModule {
    /// Absolute path to the module's executable.
    pub fn binary_path(&self) -> PathBuf {
        self.dir.join(&self.manifest.module.binary)
    }
}
