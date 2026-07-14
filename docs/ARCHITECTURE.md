# architecture

## boot sequence

```
firmware / bootloader (grub, not mine)
    |
    v
linux kernel boots, mounts initramfs
    |
    v
lunix-boot runs as pid 1
    - mounts /proc /sys /dev
    - minimal setup if anything else is needed
    |
    v
lunix-core starts, owns the terminal
    - scans modules per the module contract
    - draws the menu
    |
    v
pick a module, fork/exec, it runs, it exits, back to menu
```

no display manager, no desktop session, no login stack handed down
from somewhere else.

## why pid 1 and not just an app

running lunix-core as a normal app inside a regular desktop would
work fine technically, and it would miss the point. the thing that
matters is there's nothing else underneath, no fallback desktop
to escape to. pid 1 is the closest userspace can get to that
without writing a kernel. it comes with real responsibility, reaping
zombies, handling shutdown correctly, and that's part of why it's
worth doing properly.

## two binaries

boot/ is the init. small on purpose. gets the machine to a state
where core can run, then gets out of the way. this is not where
interesting things happen, it's plumbing, keep it short and correct.

core/ is the launcher. reads the module contract, draws the menu,
launches modules, reaps them, redraws. this is where the actual feel
of the system lives.

kept separate so boot can be hardened once and left alone while core
keeps changing.

## development loop

don't boot this on real hardware until it's proven elsewhere first.

1. build boot + core
2. boot in qemu with a custom initramfs, fast iteration, no risk
3. once stable, test on a spare partition
4. only then does it become something to boot daily

qemu is a tool here, not the destination.

## where modules fit

modules don't need to know any of this. plain executables, read
argv, do the thing, exit with a code. a module tested by running it
in a normal terminal behaves the same when launched by core at boot.
see MODULE_CONTRACT.md.
