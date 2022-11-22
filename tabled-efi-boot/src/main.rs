#![no_main]
#![no_std]
#![feature(abi_efiapi)]

//use log::info;

use uefi::prelude::*;
use uefi_services::{println, print};


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
      print!("EFI> {} = {:p} = {:#08x} = ", config_entry.guid, config_entry.address, addr_val);
      let mut str_addr = config_entry.address as *const u8;
      loop {
        if str_addr.is_null() {
          break;
        }
        if unsafe { *str_addr } == 0u8 {
          break;
        }
        print!("{}", unsafe { (*str_addr) as char } );
        str_addr = unsafe { str_addr.add(1) };
      }
      println!();
    }


    println!("Hello world!");

    system_table.boot_services().stall(99_000_000);

    Status::SUCCESS
}


