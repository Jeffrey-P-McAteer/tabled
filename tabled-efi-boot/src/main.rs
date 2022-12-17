#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(negative_impls)]

//use log::info;

use uefi::prelude::*;

use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};
use uefi::table::boot::{BootServices, OpenProtocolAttributes, OpenProtocolParams};
use uefi::{proto::Protocol, unsafe_guid};

use uefi_services::{println, print};

#[unsafe_guid("31878C87-0B75-11D5-9A4F-0090273FC14D")]
#[derive(Protocol)]
struct TabledEfiMouseProtocol {

}


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
      
      // Pick largest resolution
      let mut largest_mode = None;
      let mut ideal_mode = None;
      
      let mut largest_w = 0;
      let mut largest_h = 0;

      for mode in gop.modes() {
        let (mode_w, mode_h) = mode.info().resolution();
        println!("mode = {},{}", mode_w, mode_h);
        if mode_w == 1024 && mode_h == 768 {
          ideal_mode = Some(mode);
        }
        else if mode_w * mode_h > largest_w * largest_h {
          largest_mode = Some(mode);
          largest_w = mode_w;
          largest_h = mode_h;
        }
      }

      println!("largest mode = {},{}", largest_w, largest_h);
      
      if let Some(mode) = ideal_mode {
        gop.set_mode(&mode).expect("Failed to set graphics mode");
      }
      else if let Some(mode) = largest_mode {
        gop.set_mode(&mode).expect("Failed to set graphics mode");
      }
      else {
        println!("No graphics modes/resolutions on this machine!");
      }

      println!("Done!");
      
      // Track mouse cursor & paint pixels

/*
  #define EFI_SIMPLE_POINTER_PROTOCOL_GUID \
  { \
    0x31878c87, 0xb75, 0x11d5, {0x9a, 0x4f, 0x0, 0x90, 0x27, 0x3f, 0xc1, 0x4d } \
  }
*/    
      //  See https://github.com/Sentinel-One/efi_fuzz/blob/master/guids.csv
      //let EfiSimplePointerProtocolGuid = "EfiSimplePointerProtocolGuid";
      //let EfiSimplePointerProtocolGuid = guid!("31878C87-0B75-11D5-9A4F-0090273FC14D");

      match bs.open_protocol_exclusive::<TabledEfiMouseProtocol>(image_handle) {
        Ok(mouse_protocol) => {
          println!("Got mouse protocol!");
          


        }
        Err(e) => {
          println!("Could not open mouse protocol!: e={:?}", e);
        }
      }

      

    }
    else {
      println!("No graphics support on this machine!");
    }

    println!("Done!");

    bs.stall(99_000_000);

    Status::SUCCESS
}


