use x86_64::instructions::port::Port;

const RTC_ADDRESS_PORT: u16 = 0x70;
const RTC_DATA_PORT: u16 = 0x71;

// RTC Registers
const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY_OF_WEEK: u8 = 0x06; // Sunday = 1, ...
const RTC_DAY_OF_MONTH: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;
const RTC_CENTURY: u8 = 0x32; // Optional, depends on RTC
const RTC_STATUS_A: u8 = 0x0A;
const RTC_STATUS_B: u8 = 0x0B;

// Status Register A Flags
const RTC_UIP_FLAG: u8 = 0x80; // Update In Progress

// Status Register B Flags
const RTC_FORMAT_BINARY: u8 = 0x04; // Data in binary format (if set)
const RTC_FORMAT_24HOUR: u8 = 0x02; // 24-hour mode (if set)

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

fn read_rtc_register(reg: u8) -> u8 {
    let mut addr_port = Port::new(RTC_ADDRESS_PORT);
    let mut data_port = Port::new(RTC_DATA_PORT);

    unsafe {
        addr_port.write(reg);
        data_port.read()
    }
}

fn wait_for_rtc_update_done() {
    // Wait until UIP bit (Update In Progress) in Status Register A is clear
    while (read_rtc_register(RTC_STATUS_A) & RTC_UIP_FLAG) != 0 {
        // Spin wait
        core::hint::spin_loop();
    }
}

fn bcd_to_binary(bcd_val: u8) -> u8 {
    (bcd_val & 0x0F) + ((bcd_val >> 4) * 10)
}

pub fn get_datetime() -> DateTime {
    wait_for_rtc_update_done();

    let mut second = read_rtc_register(RTC_SECONDS);
    let mut minute = read_rtc_register(RTC_MINUTES);
    let mut hour = read_rtc_register(RTC_HOURS);
    let mut day = read_rtc_register(RTC_DAY_OF_MONTH);
    let mut month = read_rtc_register(RTC_MONTH);
    let mut year = read_rtc_register(RTC_YEAR);
    // It's common to read century separately if RTC supports it, e.g. ACPI PM Timer
    // For standard CMOS RTC, year is 00-99. Assume 20xx for now.
    // let century = read_rtc_register(RTC_CENTURY); // Needs BCD conversion too

    let status_b = read_rtc_register(RTC_STATUS_B);
    let is_binary_mode = (status_b & RTC_FORMAT_BINARY) != 0;
    let is_24_hour_mode = (status_b & RTC_FORMAT_24HOUR) != 0;

    if !is_binary_mode {
        second = bcd_to_binary(second);
        minute = bcd_to_binary(minute);
        // Hour BCD conversion needs to preserve AM/PM bit if 12-hour mode
        let hour_val = read_rtc_register(RTC_HOURS); // Reread for AM/PM bit
        hour = bcd_to_binary(hour_val & 0x7F); // Mask out AM/PM bit for conversion
        day = bcd_to_binary(day);
        month = bcd_to_binary(month);
        year = bcd_to_binary(year);
        // year_full = bcd_to_binary(century) * 100 + bcd_to_binary(year);
    }

    // Handle 12-hour to 24-hour conversion
    // This must be done AFTER BCD conversion if hour was in BCD
    if !is_24_hour_mode {
        let am_pm_bit_set = (read_rtc_register(RTC_HOURS) & 0x80) != 0; // Read original hour for AM/PM if BCD
                                                                  // or use the already binary 'hour' if not BCD.
        // If it was BCD, the 'hour' variable here is already 1-12 (binary).
        // If it was binary, it might be 1-12 (12hr) or 0-23 (24hr).
        // The RTC_HOURS register gives PM status if bit 7 is set (only in 12hr mode)
        
        let is_pm = if is_binary_mode {
            // In binary 12-hour mode, hour register directly has PM bit if RTC supports it.
            // However, many RTCs just give 1-12 and AM/PM is separate or implied.
            // The check `(read_rtc_register(RTC_HOURS) & 0x80) != 0` is more standard for BCD 12hr.
            // For simplicity, let's assume if not 24hr and binary, it is 1-12 for hour and we need to infer AM/PM if possible
            // or rely on a typical configuration (e.g. some systems might just give 1-12 and you don't get AM/PM bit separately in binary mode).
            // The original code for BCD handling of AM/PM bit is safer:
            (read_rtc_register(RTC_HOURS) & 0x80) != 0 
        } else {
            // If BCD, we used `hour_val` before which was the raw BCD hour read.
            (read_rtc_register(RTC_HOURS) & 0x80) != 0
        };

        if hour == 12 { // Special handling for 12 AM (midnight) and 12 PM (noon)
            if !is_pm { // 12 AM
                hour = 0;
            }
            // if is_pm, it's 12 PM, hour remains 12. 
        } else if is_pm {
            hour = (hour + 12) % 24; // For 1 PM to 11 PM
        }
        // If not PM and not 12, hour is already correct (1 AM to 11 AM)
    }

    // Assuming current century is 2000. Add 2000 to the two-digit year.
    let current_year: u16 = 2000 + u16::from(year);

    DateTime {
        year: current_year,
        month,
        day,
        hour,
        minute,
        second,
    }
} 