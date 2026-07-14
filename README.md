# lunix

```
 _              _
| |_   _ _ __  (_)_  __
| | | | | '_ \ | \ \/ /
| | |_| | | | || |>  <
|_|\__,_|_| |_|/_/_/\_\
```

custom linux userspace. own init, own launcher, own tools.
runs on the linux kernel, everything above that is mine.

## what this is

lunix replaces systemd and the usual desktop stack with something
i built and understand end to end. kernel boots, my init runs as
pid 1, mounts what needs mounting, hands off to my launcher. no
login manager, no desktop environment, no service manager doing
fifty things i never asked for.

on top of that sits everything else: tools, modules, whatever i
build next. each one does one thing. each one works standalone if
you pull it out and run it somewhere else. none of it is required
to make the next piece work.

base system is debian 12. not because debian is exciting, because
it's stable and out of the way. the kernel and base tools come from
there, everything above pid 1 is mine.

right now this is 64 bit userspace on the stock linux kernel. a
kernel of my own is on the list but it's a long way off, and
nothing here depends on that ever happening.

## philosophy

software today ships bloated by default. every tool wants a
plugin system before it does the one thing it was for. i'd rather
write less and understand all of it. small tools, sharp edges, no
dependency i didn't choose myself. build it small, get it working,
use it daily, add the next piece when there's an actual reason to.
that's the whole method. no roadmap slides, no grand unveiling,
just one working thing after another.

## layout

```
lunix/
    boot/               pid 1, mounts what's needed, hands off to core
    core/               reads modules, shows a menu, runs what you pick
    modules/
        networking/     network tools and modules
        graphics/       rendering, display, anything visual
        audio/          sound
        tools/          everything else, RE tools, utilities, etc
    docs/               architecture and module contract, technical only
```

each module category folder holds submodules, added one at a time as
they get built. each module is its own repo with its own README.
this repo doesn't explain them, it just says where they live.

## languages

boot and core: rust. modules: c by default, go for
networking/security tools, asm inside a module where it earns its
place. never asm as a whole module.

## Prereq
```sh
sudo apt install debootstrap squashfs-tools grub-pc-bin grub-efi-amd64-bin xorriso busybox-static
```

## building

boot and core are separate crates in one cargo workspace:

```sh
cargo build -p lunix-boot
cargo build -p lunix-core
```

see boot/README.md and core/README.md for what each actually does
right now, and docs/ for the module contract and boot pipeline.

## status

see STATUS.md. checked box means it runs, not that it's planned.

## license

MIT.
