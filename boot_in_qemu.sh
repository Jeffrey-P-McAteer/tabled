#!/bin/bash

set -ex

HOST_TARGET=''
case $(lscpu | grep Architecture | awk '{ print $NF }') in
    x86_64) HOST_TARGET='x86_64-unknown-uefi' ;;
    aarch64) HOST_TARGET='aarch64-unknown-uefi' ;;
    *) echo "Architecture $(lscpu | grep Architecture | awk '{ print $NF }') not supported!" ; exit 1 ;;
esac

TARGET=${TARGET:-$HOST_TARGET}

cargo build --release --target=$TARGET


# Find ovmf files, copy to ./target/vm/
mkdir -p target/vm/

if ! [ -e target/vm/OVMF_CODE.fd  ] ; then
  find /usr -path '*x64*' -iname 'OVMF_CODE.fd' -print -exec cp '{}' target/vm/OVMF_CODE.fd \; 2>/dev/null || true
fi

if ! [ -e target/vm/OVMF_VARS.fd  ] ; then
  find /usr -path '*x64*' -iname 'OVMF_VARS.fd' -print -exec cp '{}' target/vm/OVMF_VARS.fd \; 2>/dev/null || true
fi

mkdir -p target/vm/efi_boot_contents/

mkdir -p target/vm/efi_boot_contents/EFI/BOOT

# Matches what most firmware looks for to execute by default
cp ./target/$TARGET/release/tabled-efi-boot.efi target/vm/efi_boot_contents/EFI/BOOT/BOOTX64.EFI


QEMU_VM_BIN=qemu-system-x86_64
case $TARGET in
    x86_64-unknown-uefi) QEMU_VM_BIN='qemu-system-x86_64' ;;
    aarch64-unknown-uefi) QEMU_VM_BIN='qemu-system-aarch64' ;;
    *) echo "No known QEMU system binary for $TARGET, cannot boot VM" ; exit 1 ;;
esac

echo ''
echo ''
echo 'Ctrl+Alt+G to escape VM'
echo ''
echo ''


$QEMU_VM_BIN -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=target/vm/OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=target/vm/OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:target/vm/efi_boot_contents

