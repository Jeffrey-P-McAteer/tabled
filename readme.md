
# Tabled

_A kernel designed around SQLite tables instead of filesystems_


# Goals

 - Modify `sqlite-trunk` to work with block devices and provide fairly sophisticated table overlay capabilities, even across block devices (viewed as sqlite "database"s)
 - Write an EFI program to:
    - Read addtl config text file off program's origin FS
    - Enumerate block devices
    - Load a kernel from a sqlite database in a block device
    - Pass control to the kernel
 - Write a kernel to:
    - Provide database access APIs
    - Verify some 3rd-party driver code from database tables
        - Count loops, branches, prove a number of correctness guarantees using nothing more than machine code
    - Load verified 3rd-party driver code
    - Load un-verified 3rd-party driver code as a WASM blob
        - Essentially allowing any 3rd-party code, but badly-written code will get a performance hit. Proven code may be loaded w/o any safety/permissions checks, assuming the validator can be trusted.
    - All hardware drivers communicate through in-memory database tables
    - 


# Non-Goals

 - 32 bit support




# Building

See `boot_in_qemu.sh` for individual sub-component build commands; this script should be a oneshot setup-and-go build+run step
assuming rustup/cargo/qemu are installed.

```bash
./boot_in_qemu.sh

# detects host architecture & builds for that,
# see ./target/x86_64-unknown-uefi/release/tabled-efi-boot.efi
# or  ./target/aarch64-unknown-uefi/release/tabled-efi-boot.efi

# Override by setting the TARGET environment variable:
TARGET=aarch64-unknown-uefi ./boot_in_qemu.sh
TARGET=x86_64-unknown-uefi ./boot_in_qemu.sh

```








