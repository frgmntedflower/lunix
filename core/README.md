# lunix-core

The home. Discovers modules per
[`../docs/MODULE_CONTRACT.md`](../docs/MODULE_CONTRACT.md), draws the
menu, launches whatever you pick, waits for it to come back.

## status

The discovery and launch logic (`src/discover.rs`, `src/launch.rs`,
`src/manifest.rs`) is written and should actually work — parses
`lunix.toml`, walks `~/.lunix/modules/`, forks/execs modules,
surfaces failures instead of swallowing them.

The menu itself (`src/main.rs`) is deliberately plain right now — a
numbered text list, not a TUI. That's next, once this plain version
is proven against a couple of real modules. Get the boring skeleton
right before making it pretty.

## running it locally (no boot pipeline needed)

lunix-core doesn't require PID-1/init to run — that's the whole point
of keeping module execution decoupled from boot (see
`../docs/ARCHITECTURE.md`). You can run it in any normal terminal:

```sh
mkdir -p ~/.lunix/modules
cargo run
```

With no modules installed yet you'll just see an empty menu pointing
you at the module contract. Drop a directory with a `lunix.toml` and a
binary into `~/.lunix/modules/<name>/` and it'll show up on the next
loop.

## next steps

- [ ] first real module to test discovery/launch against something
      that isn't hypothetical
- [ ] swap the plain text menu for a retro TUI (ratatui) once the
      above is proven
- [ ] `mode = "detached"` support, once something actually needs it
