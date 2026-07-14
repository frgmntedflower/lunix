# module contract

written before any module exists. the rules of the world, everything
built after has to fit inside this, not the other way around.

status: draft v0.1, will change once boot/core are real and reality
pushes back. what matters is there's always a current written
contract, never an implicit one.

## what a module is

1. a single executable binary
2. a manifest file next to it
3. nothing else required, no shared libs, no dependency on core's
   internals. delete core entirely and every module still runs from
   a terminal on its own.

## the manifest

every module ships a lunix.toml next to its binary:

```toml
[module]
name = "lunix-scope"
version = "0.1.0"
summary = "process and memory inspector"
binary = "lunix-scope"
icon = "scope"
category = "tools"

[module.launch]
mode = "foreground"
args = []
```

category is for menu grouping only, matches the modules/ folder
names (networking, graphics, audio, tools). unrecognized categories
fall under tools until core knows about them, adding a category
never breaks an old manifest.

## discovery

core scans ~/.lunix/modules/ at startup, one directory per module:

```
~/.lunix/modules/<name>/
    lunix.toml
    <binary>
```

no registry, no network call. add a module by dropping a directory
in.

## launching

foreground: core hands off the terminal, fork, exec, wait. module
exits, core redraws. default, covers almost everything.

detached: fork, exec, don't wait. defined now, not implemented yet,
so the manifest format doesn't need to change later when something
needs it.

no ipc, no plugin api in v0.1. core starts a process and gets out of
the way. anything fancier is a deliberate later decision.

## exit contract

exit 0, clean, back to menu. anything else, core shows the code and
waits for a keypress instead of silently redrawing. failures stay
visible.

## changelog

v0.1: single binary modules, toml manifest, directory scan discovery,
foreground/detached modes, exit code error signaling.
