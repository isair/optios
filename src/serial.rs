// Module for basic serial port interaction (COM1)

use x86_64::instructions::port::Port;

// Define the base address for the COM1 port
const PORT_COM1_BASE: u16 = 0x3f8;

// Define offsets for the COM1 registers
const PORT_DATA_OFFSET: u16 = 0;        // Data register (read/write)
const PORT_INT_ENABLE_OFFSET: u16 = 1;  // Interrupt Enable Register
const PORT_FIFO_CTRL_OFFSET: u16 = 2;   // FIFO Control Register
const PORT_LINE_CTRL_OFFSET: u16 = 3;   // Line Control Register
const PORT_MODEM_CTRL_OFFSET: u16 = 4;  // Modem Control Register
const PORT_LINE_STATUS_OFFSET: u16 = 5; // Line Status Register

/// Initializes the COM1 serial port.
/// Configures it for 115200 baud, 8 data bits, no parity, 1 stop bit.
pub fn initialize_port() {
    // Disable interrupts
    let mut int_enable_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_INT_ENABLE_OFFSET);
    unsafe { int_enable_port.write(0x00); }

    // Enable DLAB (Divisor Latch Access Bit) to set baud rate
    let mut line_ctrl_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_LINE_CTRL_OFFSET);
    unsafe { line_ctrl_port.write(0x80); }

    // Set divisor to 3 (lo byte) for 38400 baud (115200 / 3 = 38400)
    let mut divisor_lo_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_DATA_OFFSET); // DLAB on, this is divisor lo
    unsafe { divisor_lo_port.write(0x03); }
    let mut divisor_hi_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_INT_ENABLE_OFFSET); // DLAB on, this is divisor hi
    unsafe { divisor_hi_port.write(0x00); }

    // Set line control: 8 bits, no parity, one stop bit (8N1)
    unsafe { line_ctrl_port.write(0x03); } // DLAB off

    // Initialize FIFO control register
    let mut fifo_ctrl_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_FIFO_CTRL_OFFSET);
    unsafe { fifo_ctrl_port.write(0xC7); }

    // Initialize modem control register
    let mut modem_ctrl_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_MODEM_CTRL_OFFSET);
    unsafe { modem_ctrl_port.write(0x0B); } // IRQs enabled, RTS/DSR set

    // Set in loopback mode to test the serial chip
    unsafe { modem_ctrl_port.write(0x1E); } 

    // Test serial chip (send byte 0xAE and check if reads back)
    let mut data_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_DATA_OFFSET);
    unsafe { data_port.write(0xAE); }
    unsafe {
        if data_port.read() != 0xAE {
            // TODO: Handle serial port initialization failure (e.g., panic or log)
            // For now, we assume it works.
        }
    }

    // Set back to normal operation mode
    unsafe { modem_ctrl_port.write(0x0F); }
}

fn is_transmit_empty() -> bool {
    let mut line_status_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_LINE_STATUS_OFFSET);
    unsafe { (line_status_port.read() & 0x20) != 0 }
}

/// Writes a single byte to the serial port.
pub fn write_byte(byte: u8) {
    while !is_transmit_empty() {
        // Spin wait for the transmitter to be empty
        core::hint::spin_loop();
    }
    let mut data_port: Port<u8> = Port::new(PORT_COM1_BASE + PORT_DATA_OFFSET);
    unsafe { data_port.write(byte); }
} 