//! lunix-core — the home.
//!
//! v0.1 is deliberately plain: a numbered text menu, not a fancy TUI.
//! The retro widget layer (ratatui, ascii glyphs, whatever) comes once
//! discovery + launch are solid and boring. Get the skeleton right
//! before dressing it up.

mod discover;
mod launch;
mod manifest;

use anyhow::Result;
use launch::LaunchOutcome;
use std::io::{self, Write};

fn main() -> Result<()> {
    loop {
        let modules = discover::discover()?;
        draw_menu(&modules);

        print!("\n> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            println!("lunix-core: later.");
            break;
        }

        let Ok(index) = input.parse::<usize>() else {
            println!("lunix-core: didn't parse that. type a number or 'q'.");
            continue;
        };

        let Some(module) = modules.get(index.wrapping_sub(1)) else {
            println!("lunix-core: no module at that index.");
            continue;
        };

        println!("lunix-core: launching {}...", module.manifest.module.name);
        match launch::launch(module) {
            Ok(LaunchOutcome::Clean) => {}
            Ok(LaunchOutcome::Failed(code)) => {
                println!(
                    "lunix-core: '{}' exited with code {code}. press enter to return.",
                    module.manifest.module.name
                );
                let mut _pause = String::new();
                io::stdin().read_line(&mut _pause)?;
            }
            Err(e) => {
                println!("lunix-core: failed to launch: {e:#}");
            }
        }
    }

    Ok(())
}

fn draw_menu(modules: &[manifest::ResolvedModule]) {
    println!("\n=== lunix ===");
    if modules.is_empty() {
        println!("(no modules found in {:?})", discover::modules_dir());
        println!("drop a module in there — see docs/MODULE_CONTRACT.md");
    } else {
        for (i, m) in modules.iter().enumerate() {
            println!(
                "  [{}] {:<20} {}",
                i + 1,
                m.manifest.module.name,
                m.manifest.module.summary
            );
        }
    }
    println!("  [q] quit");
}
