#!/bin/bash
set -e
cd "$(dirname "$0")"
BASE="$(pwd)"

if [ "$(id -u)" -ne 0 ]; then
    echo "run as root (debootstrap/chroot/mount need it)"
    exit 1
fi

MISSING=""
for tool in debootstrap mksquashfs grub-mkrescue xorriso cargo; do
    command -v "$tool" >/dev/null || MISSING="$MISSING $tool"
done
if [ -n "$MISSING" ]; then
    echo "missing tools:$MISSING"
    echo "try: apt install cargo mtools debootstrap squashfs-tools grub-pc-bin grub-efi-amd64-bin xorriso"
    exit 1
fi

WORK="$BASE/work"
ROOTFS="$WORK/rootfs"
ISOROOT="$WORK/iso-root"
OUTPUT="$BASE/output"

echo "cleaning up old data"
rm -rf "$WORK"
rm -rf "$OUTPUT"
mkdir -p "$OUTPUT"

echo "== stage 1: base rootfs =="
debootstrap --variant=minbase bookworm "$ROOTFS" http://deb.debian.org/debian
mount --bind /dev "$ROOTFS/dev"
mount --bind /proc "$ROOTFS/proc"
mount --bind /sys "$ROOTFS/sys"
chroot "$ROOTFS" apt-get update
chroot "$ROOTFS" apt-get install -y --no-install-recommends linux-image-amd64
umount -lf "$ROOTFS/dev" "$ROOTFS/proc" "$ROOTFS/sys"

echo "== stage 2: build lunix binaries (musl, static) =="
REAL_USER="${SUDO_USER:-$USER}"
sudo -u "$REAL_USER" -H bash -c '
    export PATH="$HOME/.cargo/bin:$PATH"
    cd "'"$BASE"'/.." &&
    rustup target add x86_64-unknown-linux-musl 2>/dev/null || true
    cargo build --release --target x86_64-unknown-linux-musl -p lunix-boot &&
    cargo build --release --target x86_64-unknown-linux-musl -p lunix-core
'

echo "== stage 3: install lunix into rootfs =="
install -m 755 ../target/x86_64-unknown-linux-musl/release/lunix-boot "$ROOTFS/sbin/init"
install -D -m 755 ../target/x86_64-unknown-linux-musl/release/lunix-core "$ROOTFS/usr/local/bin/lunix-core"


echo "== stage 4: squash the rootfs =="
mkdir -p "$ISOROOT/live" "$ISOROOT/boot/grub"
mksquashfs "$ROOTFS" "$ISOROOT/live/filesystem.squashfs" -noappend -comp xz

echo "== stage 5: kernel =="
KERNEL=$(ls "$ROOTFS"/boot/vmlinuz-* | head -n1)
cp "$KERNEL" "$ISOROOT/boot/vmlinuz"

echo "== stage 6: stage1 initramfs =="
./build-stage1-initramfs.sh
cp work/initrd.img "$ISOROOT/boot/initrd.img"

echo "== stage 7: grub config =="
cp grub.cfg "$ISOROOT/boot/grub/grub.cfg"

echo "== stage 8: build iso =="
grub-mkrescue -o "$OUTPUT/lunix.iso" "$ISOROOT"

echo "done: $OUTPUT/lunix.iso"
