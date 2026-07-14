//! lunix-boot — PID 1.
//!
//! Job, in order:
//!   1. mount essential filesystems
//!   2. do whatever minimal hardware/network bring-up is required
//!   3. exec into lunix-core, handing off the terminal
//!   4. if lunix-core ever exits, reap zombies and decide what happens next
//!      (v0.1: just restart it — you're never dumped at a dead prompt)
//!
//! This file stays small on purpose. Every line here runs before anything
//! else on the machine can. Boring and correct beats clever every time.

fn main() {
    println!("lunix-boot: starting as pid {}", std::process::id());

    // step 1: mount essential filesystems.
    // TODO: mount /proc, /sys, /dev via nix::mount before anything else
    // touches the filesystem. Without /proc in particular, most tools
    // (including lunix-core's own module discovery later) won't work.
    mount_essential_filesystems();

    // step 2: minimal bring-up.
    // TODO: whatever's needed before core can run — likely nothing at
    // first. Resist adding things here "just in case." If lunix-core
    // needs it, lunix-core should own bringing it up, not init.

    // step 3: hand off to lunix-core.
    // v0.1: just loop-exec it so a crash doesn't leave the machine dead.
    run_core_forever();
}

fn mount_essential_filesystems() {
    // TODO: implement with nix::mount::mount()
    // - proc  -> /proc
    // - sysfs -> /sys
    // - devtmpfs -> /dev
    println!("lunix-boot: TODO mount /proc /sys /dev");
}

fn run_core_forever() {
    // TODO: fork/exec lunix-core, reap it with waitpid, loop.
    // As PID 1, this process is also responsible for reaping ALL
    // orphaned zombie processes system-wide, not just lunix-core's
    // direct children — that's part of what makes PID 1 different from
    // an ordinary process. Handle SIGCHLD accordingly once this is real.
    println!("lunix-boot: TODO exec lunix-core, restart on exit");
}
