# lunix-boot

The init. PID 1. See [`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md)
for why this exists as its own tiny binary instead of folding into
lunix-core.

## status

Skeleton. Every step is a `TODO` with the intent written down —
nothing here pretends to work yet. See `src/main.rs`.

## scope, on purpose

This binary does exactly three things and nothing else:

1. mount `/proc`, `/sys`, `/dev`
2. whatever minimal bring-up lunix-core actually needs (nothing, so far)
3. run lunix-core, forever, reaping it if it dies

If you find yourself adding a fourth kind of thing here, it probably
belongs in lunix-core instead. Init should stay boring.

## testing without touching real hardware

Don't boot this on your daily driver until it's proven in a throwaway
environment first:

1. Build a minimal initramfs with `lunix-boot` as `/init`.
2. Boot it in QEMU: `qemu-system-x86_64 -kernel <kernel> -initrd
   <initramfs> -append "console=ttyS0" -nographic`
3. Iterate there. Only once it's boring and reliable does it earn a
   real partition.

(Exact build steps for the initramfs will get written up here once
step 1 of `src/main.rs` — mounting filesystems — is actually
implemented instead of a TODO.)
