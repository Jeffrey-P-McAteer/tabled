#![no_main]
#![no_std]
#![feature(abi_efiapi)]

//use log::info;

use uefi::prelude::*;
use uefi_services::println;


#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Read uefi data into system_table
    if let Err(e) = uefi_services::init(&mut system_table) {
      return Status::ABORTED;
    }

    // Dump all EFI variable data
    for ref config_entry in system_table.config_table() {
      let addr_val: u64 = if ! config_entry.address.is_null() {
        unsafe { *(config_entry.address as *const u64 ) }
      } else { 0 };
      println!("EFI> {} = {:p} = {:#08x}", config_entry.guid, config_entry.address, addr_val);
    }


    println!("Hello world!");

    system_table.boot_services().stall(18_000_000);

    Status::SUCCESS
}


