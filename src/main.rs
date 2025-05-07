#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_x86_interrupt)]

extern crate rlibc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

mod serial;
pub mod vga_text;
mod rtc;

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

entry_point!(kernel_entry);

fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    println!("Kernel Entry via main.rs -> kernel_entry");
    kernel_main(boot_info);
}

fn initialize_serial() {
    serial::initialize_port();
}

fn print_to_serial(message: &[u8]) {
    for &byte in message {
        serial::write_byte(byte);
    }
    serial::write_byte(b'\n');
}

pub fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    initialize_serial();
    print_to_serial(b"Serial Initialized (from kernel_main).");

    // Set a global background color to Black and default foreground to LightGray
    vga_text::set_text_color(vga_text::Color::LightGray, vga_text::Color::Black);
    vga_text::clear_screen(); // Clear screen with new background

    // Futuristic Welcome Header (using simpler ASCII for compatibility)
    vga_text::set_text_color(vga_text::Color::LightBlue, vga_text::Color::Black);
    println!("+--------------------------------------------------------------+");
    println!("|                                                              |");
    vga_text::set_text_color(vga_text::Color::White, vga_text::Color::Black);
    println!("|                Welcome to OptiOS v0.1.0                      |");
    vga_text::set_text_color(vga_text::Color::LightBlue, vga_text::Color::Black);
    println!("|                                                              |");
    println!("+--------------------------------------------------------------+");
    println!(); // Empty line for spacing

    // Display current time
    let datetime = rtc::get_datetime();
    vga_text::set_text_color(vga_text::Color::LightGreen, vga_text::Color::Black);
    println!(
        "    System Time: {}-{:02}-{:02} {:02}:{:02}:{:02}",
        datetime.year, datetime.month, datetime.day,
        datetime.hour, datetime.minute, datetime.second
    );
    println!(); // Empty line for spacing

    // Boot messages
    vga_text::set_text_color(vga_text::Color::Cyan, vga_text::Color::Black);
    println!("    VGA Display Initialized.");
    println!("    Bootloader sequence complete.");
    println!("    Initializing kernel modules...");
    
    print_to_serial(b"OptiOS Booting via Bootloader..."); // Serial logs can remain as is

    // Final message before halt
    vga_text::set_text_color(vga_text::Color::Pink, vga_text::Color::Black);
    println!();
    println!("    System Core Halting. CPU going to sleep.");
    print_to_serial(b"Halting CPU...");
    halt_loop();
}

pub fn halt_loop() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC! (via main.rs)");
    if let Some(location) = info.location() {
        println!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        );
    } else {
        println!("Panic occurred but location information is unavailable.");
    }
    if let Some(message) = info.payload().downcast_ref::<&str>() {
         println!("Panic payload: {}", message);
    } else {
         println!("Panic payload: <not a string>");
    }

    print_to_serial(b"KERNEL PANIC!");

    halt_loop();
}

#[lang = "eh_personality"] extern "C" fn eh_personality() {} 