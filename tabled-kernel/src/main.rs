#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;

use core::panic::PanicInfo;
#[panic_handler] // TODO better
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
  // TODO
  Status::SUCCESS
}



