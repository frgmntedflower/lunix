//! lunix-boot — pid 1.
//!
//! job, in order:
//!   1. mount essential filesystems
//!   2. do whatever minimal hardware/network bring-up is required
//!   3. exec into lunix-core, handing off the terminal
//!   4. if lunix-core ever exits, reap zombies and decide what happens next
//!      (v0.1: just restart it — you're never dumped at a dead prompt)
//!
//! this file stays small on purpose. every line here runs before anything
//! else on the machine can. boring and correct beats clever every time.

use exec::Command;
use fork::{fork, waitpid, Fork};
use nix::mount::{mount, MsFlags};

const LUNIX_CORE_BIN_PATH: &str = "/usr/local/bin/lunix-core";

fn main() {
    println!("lunix-boot: starting as pid {}", std::process::id());

    // step 1: mount essential filesystems.
    // todo: mount /proc, /sys, /dev via nix::mount before anything else
    // touches the filesystem. without /proc in particular, most tools
    // (including lunix-core's own module discovery later) won't work.
    mount_essential_filesystems();

    // step 2: minimal bring-up.
    // todo: whatever's needed before core can run — likely nothing at
    // first. resist adding things here "just in case." if lunix-core
    // needs it, lunix-core should own bringing it up, not init.

    // step 3: hand off to lunix-core.
    // v0.1: just loop-exec it so a crash doesn't leave the machine dead.
    run_core_forever();
}

fn mount_essential_filesystems() {
    let result_proc = mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    );

    match result_proc {
        Ok(()) => println!("lunix-boot: mount /proc ... ok"),
        Err(e) => eprintln!("lunix-boot: mount /proc failed: {e}"),
    }

    let result_sys = mount(
        Some("sysfs"),
        "/sys",
        Some("sysfs"),
        MsFlags::empty(),
        None::<&str>,
    );

    match result_sys {
        Ok(()) => println!("lunix-boot: mount /sys ... ok"),
        Err(e) => eprintln!("lunix-boot: mount /sys failed: {e}"),
    }

    let result_dev = mount(
        Some("devtmpfs"),
        "/dev",
        Some("devtmpfs"),
        MsFlags::empty(),
        None::<&str>,
    );

    match result_dev {
        Ok(()) => println!("lunix-boot: mount /dev ... ok"),
        Err(e) => eprintln!("lunix-boot: mount /dev failed: {e}"),
    }

    let result_tmp = mount(
        Some("tmpfs"),
        "/tmp",
        Some("tmpfs"),
        MsFlags::empty(),
        None::<&str>,
    );

    match result_tmp {
        Ok(()) => println!("lunix-boot: mount /tmp ... ok"),
        Err(e) => eprintln!("lunix-boot: mount /tmp failed: {e}"),
    }
}

fn run_core_forever() {
    // todo: fork/exec lunix-core, reap it with waitpid, loop.
    // as pid 1, this process is also responsible for reaping all
    // orphaned zombie processes system-wide, not just lunix-core's
    // direct children — that's part of what makes pid 1 different from
    // an ordinary process. handle sigchld accordingly once this is real.
    loop {
        match fork() {
            Ok(Fork::Parent(child_pid)) => {
                println!("lunix-boot: forked child with pid {child_pid}");
                match waitpid(child_pid) {
                    Ok(status) => println!("Child exited with status: {status}"),
                    Err(err) => println!("Child failed with error: {err}"),
                }
            }
            Ok(Fork::Child) => {
                let err = Command::new(LUNIX_CORE_BIN_PATH).exec();
                println!("Error: {}", err);
            }
            Err(e) => {
                eprintln!("lunix-boot: fork failed: {e}");
            }
        }
    }
}
