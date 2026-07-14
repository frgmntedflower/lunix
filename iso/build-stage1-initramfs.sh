#!/bin/bash
set -e
cd "$(dirname "$0")"
BASE="$(pwd)"
WORK=work/stage1
rm -rf "$WORK"
mkdir -p "$WORK/bin" "$WORK/proc" "$WORK/sys" "$WORK/dev" "$WORK/mnt/cdrom" "$WORK/mnt/root"
BUSYBOX_BIN=$(command -v busybox || true)
if [ -z "$BUSYBOX_BIN" ]; then
    apt-get install -y busybox-static
    BUSYBOX_BIN=/bin/busybox
fi
cp "$BUSYBOX_BIN" "$WORK/bin/busybox"
KVER=$(ls "$BASE/work/rootfs/lib/modules")
MODSRC="$BASE/work/rootfs/lib/modules/$KVER/kernel/drivers"
mkdir -p "$WORK/lib/modules"
for mod in \
    "$MODSRC/scsi/scsi_common.ko" \
    "$MODSRC/scsi/scsi_mod.ko" \
    "$MODSRC/ata/libata.ko" \
    "$MODSRC/cdrom/cdrom.ko" \
    "$MODSRC/ata/ata_piix.ko" \
    "$MODSRC/scsi/sr_mod.ko" \
    "$BASE/work/rootfs/lib/modules/$KVER/kernel/fs/isofs/isofs.ko" \
    "$MODSRC/block/loop.ko" \
    "$BASE/work/rootfs/lib/modules/$KVER/kernel/fs/squashfs/squashfs.ko"
do
    cp "$mod" "$WORK/lib/modules/"
done
cp stage1-init "$WORK/init"
chmod +x "$WORK/init" "$WORK/bin/busybox"
( cd "$WORK" && find . | cpio -o -H newc 2>/dev/null | gzip ) > "$BASE/work/initrd.img"
echo "stage1 initramfs built at $BASE/work/initrd.img"
