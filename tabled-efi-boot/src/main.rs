#![no_main]
#![no_std]
#![feature(abi_efiapi)]

//use log::info;

use uefi::prelude::*;

use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};
use uefi::table::boot::{BootServices, OpenProtocolAttributes, OpenProtocolParams};

use uefi_services::{println, print};


#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
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

    // Can we detect CPU cores/ram amount? Enumerate disks?
    let mut bs = system_table.boot_services();

    let mm = bs.memory_map_size();
    println!("RAM> entry_size={}, map_size={} ", mm.entry_size, mm.map_size);

    println!("Hello world!");
    bs.stall(1_000_000);

    print!("Starting graphics in ");
    for s in (1..4).rev() {
      print!("{}", s);
      for d in 0..3 {
        print!(".");
        bs.stall(1_000_000 / 3);
      }
    }
    println!("");
    bs.stall(1_000_000);

    // Graphics!
    if let Ok(handle) = bs.get_handle_for_protocol::<GraphicsOutput>() {
      let gop = unsafe {
        &mut bs
        .open_protocol::<GraphicsOutput>(
            OpenProtocolParams {
                handle,
                agent: image_handle,
                controller: None,
            },
            // For this test, don't open in exclusive mode. That
            // would break the connection between stdout and the
            // video console.
            OpenProtocolAttributes::GetProtocol,
        )
        .expect("failed to open Graphics Output Protocol")
      };
      
      // Todo steal more from https://github.com/rust-osdev/uefi-rs/blob/02e02d4018ad2c3cd6312dd76cfe1db51d466b26/uefi-test-runner/src/proto/console/gop.rs

      // set_graphics_mode(gop);
      // fill_color(gop);
      // draw_fb(gop);
    }
    else {
      println!("No graphics support on this machine!");
    }


    println!("Done!");
    bs.stall(99_000_000);

    Status::SUCCESS
}


