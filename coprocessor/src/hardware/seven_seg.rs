use avr_hal_generic::void::Void;
use embedded_hal::digital::v2::OutputPin;
use arduino_uno::prelude::*;

/// Converts any byte to the corresponding shift register value (or 0 if invalid)
pub fn byte_to_shift_code(byte: u8) -> u8 {
    return match byte {
        0x00 => 252,
        0x01 => 96,
        0x02 => 218,
        0x03 => 242,
        0x04 => 102,
        0x05 => 182,
        0x06 => 190,
        0x07 => 224,
        0x08 => 254,
        0x09 => 246,
        0x0a => 238,
        0x0b => 62,
        0x0c => 156,
        0x0d => 122,
        0x0e => 158,
        0x0f => 142,
        _ => 0,
    };
}

/// Write a byte to a shift register
pub fn write_byte_to_shift_register(
    byte: u8,
    clock_pin: &mut dyn OutputPin<Error = Void>,
    latch_pin: &mut dyn OutputPin<Error = Void>,
    data_pin: &mut dyn OutputPin<Error = Void>,
) {
    // Pull the register latch low to accept new data
    latch_pin.set_low().void_unwrap();

    // A simple MSB-First implementation of Arduino's shiftout() function
    for i in 0..8 {
        // Handle setting the data pin based on the current bit
        if (byte & (1 << (7 - i))) > 0 {
            data_pin.set_high().void_unwrap();
        } else {
            data_pin.set_low().void_unwrap();
        }

        // Toggle the clock
        clock_pin.set_high().void_unwrap();
        arduino_uno::delay_ms(10);
        clock_pin.set_low().void_unwrap();
    }

    // Set the latch to no longer accept data
    latch_pin.set_high().void_unwrap();
}
