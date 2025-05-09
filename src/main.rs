#![no_std]
#![no_main]
// #![feature(lang_items)] // Removed as no longer needed
// #![feature(abi_x86_interrupt)] // Likely not needed for UEFI app stage

// extern crate rlibc; // Keep for now, might be unneeded.

// use core::panic::PanicInfo; // Removed as it's unused
use uefi::prelude::*;
use log; // Import log to use its macros like info!, error!
use uefi_services; // Add this use statement
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat, Mode};
// use core::fmt::Write; // No longer needed after switching to output_string

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
};
use uefi_graphics::UefiDisplay;

// mod serial; // Comment out for now
// pub mod vga_text; // Comment out for now
// mod rtc; // Comment out for now

/*
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}
", format_args!($($arg)*)));
}
*/

// entry_point!(kernel_entry); // Removed bootloader entry point

// fn kernel_entry(boot_info: &'static BootInfo) -> ! {
//     println!("Kernel Entry via main.rs -> kernel_entry");
//     kernel_main(boot_info);
// }

// fn initialize_serial() {
//     serial::initialize_port();
// }

// fn print_to_serial(message: &[u8]) {
//     for &byte in message {
//         serial::write_byte(byte);
//     }
//     serial::write_byte(b'\n');
// }

// pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // initialize_serial();
    // print_to_serial(b"Serial Initialized (from kernel_main).");

    // Set a global background color to Black and default foreground to LightGray
    // vga_text::set_text_color(vga_text::Color::LightGray, vga_text::Color::Black);
    // vga_text::clear_screen(); // Clear screen with new background

    // Futuristic Welcome Header (using simpler ASCII for compatibility)
    // vga_text::set_text_color(vga_text::Color::LightBlue, vga_text::Color::Black);
    // println!("+--------------------------------------------------------------+");
    // println!("|                                                              |");
    // vga_text::set_text_color(vga_text::Color::White, vga_text::Color::Black);
    // println!("|                Welcome to OptiOS v0.1.0                      |");
    // vga_text::set_text_color(vga_text::Color::LightBlue, vga_text::Color::Black);
    // println!("|                                                              |");
    // println!("+--------------------------------------------------------------+");
    // println!(); // Empty line for spacing

    // Display current time
    // let datetime = rtc::get_datetime();
    // vga_text::set_text_color(vga_text::Color::LightGreen, vga_text::Color::Black);
    // println!(
    //     "    System Time: {}-{:02}-{:02} {:02}:{:02}:{:02}",
    //     datetime.year, datetime.month, datetime.day,
    //     datetime.hour, datetime.minute, datetime.second
    // );
    // println!(); // Empty line for spacing

    // Boot messages
    // vga_text::set_text_color(vga_text::Color::Cyan, vga_text::Color::Black);
    // println!("    VGA Display Initialized.");
    // println!("    Bootloader sequence complete.");
    // println!("    Initializing kernel modules...");
    
    // print_to_serial(b"OptiOS Booting via Bootloader..."); // Serial logs can remain as is

    // Final message before halt
    // vga_text::set_text_color(vga_text::Color::Pink, vga_text::Color::Black);
    // println!();
    // println!("    System Core Halting. CPU going to sleep.");
    // print_to_serial(b"Halting CPU...");
//     halt_loop();
// }

// Target resolution bounds (from tutorial Part 3)
const MAX_WIDTH: usize = 1920;
const MAX_HEIGHT: usize = 1080;
// Let's try a more common one for QEMU default, like 1024x768, as an alternative
// const MAX_WIDTH: usize = 1024;
// const MAX_HEIGHT: usize = 768;

fn print_welcome_message(display: &mut UefiDisplay) -> uefi::Result {
    // This will just clear the screen to blue for now.
    display.clear(Rgb888::BLUE).map_err(|_| uefi::Status::DEVICE_ERROR)?;
    Ok(())
}

#[entry]
fn efi_main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize logger first
    if let Err(_e) = uefi_services::init(&mut system_table) {
        // Use Output::output_string for CStr16, ignoring result for this emergency print.
        let _ = system_table.stdout().output_string(cstr16!("Error: Failed to initialize uefi_services (logger).\r\n"));
        // Loop indefinitely as we can't rely on logging or proper panic handling here.
        loop { system_table.boot_services().stall(1_000_000); }
    }
    log::set_max_level(log::LevelFilter::Info);

    log::info!("OptiOS UEFI Bootloader Initializing...");
    log::info!("Image Handle: {:?}", _image_handle);
    log::info!("UEFI Revision: {}.{}",
        system_table.uefi_revision().major(),
        system_table.uefi_revision().minor()
    );

    let bt = system_table.boot_services();
    log::info!("Attempting to initialize GOP and set mode...");

    match bt.get_handle_for_protocol::<GraphicsOutput>() {
        Ok(gop_handle) => {
            log::info!("GOP Handle acquired: {:?}", gop_handle);
            match bt.open_protocol_exclusive::<GraphicsOutput>(gop_handle) {
                Ok(mut gop) => { 
                    log::info!("GOP Protocol opened exclusively. Iterating modes...");
                    
                    let mut best_mode_idx = None::<u32>;
                    let mut best_width = 0;
                    let mut best_height = 0;

                    for (i, mode_object) in gop.modes().enumerate() {
                        // Assuming mode_object is of type uefi::proto::console::gop::Mode directly
                        let mode_info = mode_object.info();
                        let (w, h) = mode_info.resolution();
                        log::info!("Mode {}: {}x{} Format: {:?}", i, w, h, mode_info.pixel_format());

                        if mode_info.pixel_format() == PixelFormat::Rgb || mode_info.pixel_format() == PixelFormat::Bgr {
                            if (w <= MAX_WIDTH && h <= MAX_HEIGHT) && (w >= best_width && h >= best_height) {
                                if w > best_width || h > best_height || best_mode_idx.is_none() {
                                    best_mode_idx = Some(i as u32);
                                    best_width = w;
                                    best_height = h;
                                }
                            }
                        }
                        // Note: If individual mode fetching could fail and gop.modes()
                        // originally returned Result, this change bypasses that error handling.
                    }

                    if let Some(selected_idx) = best_mode_idx {
                        log::info!("Selected Mode {}: {}x{}", selected_idx, best_width, best_height);
                        match gop.query_mode(selected_idx) {
                            Ok(selected_mode_object) => { 
                                if let Err(e) = gop.set_mode(&selected_mode_object) { 
                                    log::error!("Failed to set graphics mode {}: {:?}", selected_idx, e);
                                } else {
                                    let current_mode_info = gop.current_mode_info();
                                    let mut frame_buffer = gop.frame_buffer();
                                    let stride = current_mode_info.stride();
                                    let (width, height) = current_mode_info.resolution();
                                    
                                    let mut display = UefiDisplay::new(
                                        frame_buffer.as_mut_ptr(),
                                        stride as u32,
                                        (width as u32, height as u32),
                                        &gop 
                                    );
                                    let _ = print_welcome_message(&mut display);
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to query selected mode object {}: {:?}", selected_idx, e);
                            }
                        }
                    } else {
                        log::error!("No suitable RGB/BGR graphics mode found within bounds.");
                    }
                }
                Err(e) => {
                    log::error!("Failed to open GOP protocol (exclusive): {:?}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get GOP handle: {:?}", e);
        }
    }
    
    // This log might not appear if console is redirected after set_mode
    // log::info!("OptiOS halting after graphics demo."); 
    system_table.boot_services().stall(20_000_000); // Stall for 20 seconds to see the result
    Status::SUCCESS
}

// Custom panic handler and halt_loop were removed as uefi-services provides them.
// eh_personality lang item also removed, assuming uefi-services or another dep provides it.

// Try removing eh_personality as uefi-services might provide it.
// If the build fails asking for it, we can re-add it or the lang_items feature.
// #[lang = "eh_personality"] extern "C" fn eh_personality() {} 