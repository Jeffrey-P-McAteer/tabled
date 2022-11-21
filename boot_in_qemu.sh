#!/bin/bash

set -ex

# Environment setup
HOST_TARGET=''
case $(lscpu | grep Architecture | awk '{ print $NF }') in
    x86_64) HOST_TARGET='x86_64-unknown-uefi' ;;
    aarch64) HOST_TARGET='aarch64-unknown-uefi' ;;
    *) echo "Architecture $(lscpu | grep Architecture | awk '{ print $NF }') not supported!" ; exit 1 ;;
esac

TARGET=${TARGET:-$HOST_TARGET}

QEMU_VM_BIN='qemu-system-x86_64'
QEMU_VM_ARGS=(--help)
QEMU_OVMF_FILE_PATH_SEARCH='*x64*'
EFI_BOOT_BIN_NAME='BOOTX64.EFI'
case $TARGET in
    x86_64-unknown-uefi)
      QEMU_VM_BIN='qemu-system-x86_64'
      if grep -q 'x86_64' <<<"$HOST_TARGET" ; then
        QEMU_VM_ARGS=(
          -enable-kvm -m 2048M
        )
      else
        QEMU_VM_ARGS=(
          -M virt -m 2048M
        )
      fi
      QEMU_OVMF_FILE_PATH_SEARCH='*x64*'
      EFI_BOOT_BIN_NAME='BOOTX64.EFI'
    ;;
    aarch64-unknown-uefi)
      QEMU_VM_BIN='qemu-system-aarch64'
      if grep -q 'aarch64' <<<"$HOST_TARGET" ; then
        QEMU_VM_ARGS=(
          -enable-kvm -m 2048M
        )
      else
        QEMU_VM_ARGS=(
          -M virt -m 2048M
        )
      fi
      QEMU_OVMF_FILE_PATH_SEARCH='*aarch64*'
      EFI_BOOT_BIN_NAME='BOOTAA64.EFI'
    ;;
    *) echo "No known QEMU system binary for $TARGET, cannot boot VM" ; exit 1 ;;
esac

# Build everything

cargo build --release --target=$TARGET


# Find ovmf files, copy to ./target/vm/
mkdir -p target/vm/

if ! [ -e target/vm/OVMF_CODE.$TARGET.fd  ] ; then
  find /usr -path $QEMU_OVMF_FILE_PATH_SEARCH -iname 'OVMF_CODE.fd' -print -exec cp '{}' target/vm/OVMF_CODE.$TARGET.fd \; 2>/dev/null || true
fi

if ! [ -e target/vm/OVMF_VARS.$TARGET.fd  ] ; then
  find /usr -path $QEMU_OVMF_FILE_PATH_SEARCH -iname 'OVMF_VARS.fd' -print -exec cp '{}' target/vm/OVMF_VARS.$TARGET.fd \; 2>/dev/null || true
fi

# Test & suggest install steps for OVMF stuff
if ! [ -e target/vm/OVMF_CODE.$TARGET.fd ] ; then
  echo "Failed to find an OVMF_CODE.fd file for $TARGET, please try installing your distro's edk2-ovmf package."
  echo "If your distro splits the package by VM type, ensure the variant for $TARGET gets installed."
  echo ""
  echo "For example, on Arch Linux to instal x86_64 and aarch64:"
  echo "   > git clone https://aur.archlinux.org/edk2-git.git"
  echo "   > makepkg -si"
  echo ""
  exit 1
fi

mkdir -p target/vm/efi_boot_contents/

mkdir -p target/vm/efi_boot_contents/EFI/BOOT

# Matches what most firmware looks for to execute by default
cp ./target/$TARGET/release/tabled-efi-boot.efi target/vm/efi_boot_contents/EFI/BOOT/$EFI_BOOT_BIN_NAME


echo ''
echo ''
echo 'Ctrl+Alt+G to escape VM'
echo ''
echo ''


$QEMU_VM_BIN \
    "${QEMU_VM_ARGS[@]}" \
    -drive if=pflash,format=raw,readonly=on,file=target/vm/OVMF_CODE.$TARGET.fd \
    -drive if=pflash,format=raw,readonly=on,file=target/vm/OVMF_VARS.$TARGET.fd \
    -drive format=raw,file=fat:rw:target/vm/efi_boot_contents




