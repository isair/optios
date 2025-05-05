use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use crate::{println, print}; // Import println/print, remove halt_loop

// --- PIC Initialization ---

// Define the offsets for the PICs. We choose 32-47 for IRQs 0-15.
// These must not overlap with CPU exceptions (0-31).
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// Create a static mutable instance of ChainedPics, protected by a Mutex.
// `unsafe` is required because initializing PICs can cause undefined behavior if done incorrectly.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// --- Interrupt Index Enum ---

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET, // IRQ 0
    Keyboard,             // IRQ 1 (will be PIC_1_OFFSET + 1)
    // Add other hardware interrupts here
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}


// --- IDT Initialization ---

lazy_static! {
    // Create a static IDT instance.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Set the handler for the Timer interrupt (IRQ 0).
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        // Set the handler for the Keyboard interrupt (IRQ 1).
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        // Set a handler for breakpoint exceptions (INT 3)
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // Set a handler for double faults
        // unsafe { // Requires a separate stack
        //     idt.double_fault.set_handler_fn(double_fault_handler)
        //         .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        // }

        idt
    };
}

// Loads the IDT. Must be called to activate the interrupt handlers.
pub fn init_idt() {
    IDT.load();
}

// --- Interrupt Handlers ---

// Breakpoint Handler (INT 3)
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT
{:#?}", stack_frame);
}

// Double Fault Handler (requires GDT setup for separate stack)
// extern "x86-interrupt" fn double_fault_handler(
//     stack_frame: InterruptStackFrame, _error_code: u64) -> !
// {
//     panic!("EXCEPTION: DOUBLE FAULT
// {:#?}", stack_frame);
// }

// Timer Interrupt Handler (IRQ 0)
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // For now, just print a dot to show it's working.
    // In a real OS, this would handle task switching, etc.
    // print!(".");

    // IMPORTANT: Notify the PIC that the interrupt is finished.
    // Otherwise, no more timer interrupts will be received.
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// Keyboard Interrupt Handler (IRQ 1)
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    // PS/2 controller data port
    let mut port = Port::new(0x60);

    // Read the scancode from the keyboard controller.
    let scancode: u8 = unsafe { port.read() };

    // Process the scancode using the `pc-keyboard` crate.
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    // IMPORTANT: Notify the PIC that the interrupt is finished.
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

// --- Initialization Function ---

// Initializes both the IDT and the PICs.
pub fn init() {
    init_idt(); // Load the IDT first
    unsafe { PICS.lock().initialize() }; // Initialize the PICs
    println!("Interrupts: IDT loaded, PICs initialized.");
} 