#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_x86_interrupt)]

extern crate rlibc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

mod serial;
mod interrupts;
pub mod vga_text;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
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

    vga_text::clear_screen();
    println!("VGA Initialized.");

    println!("OptiOS Booting via Bootloader...");
    print_to_serial(b"OptiOS Booting via Bootloader...");

    println!("Initializing Interrupts...");
    print_to_serial(b"Initializing Interrupts...");
    interrupts::init();
    x86_64::instructions::interrupts::enable();
    println!("Interrupts enabled.");
    print_to_serial(b"Interrupts enabled.");

    println!("Halting CPU...");
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