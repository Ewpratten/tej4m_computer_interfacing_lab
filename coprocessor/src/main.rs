#![no_std]
#![no_main]

mod hardware;

use crate::hardware::seven_seg::{byte_to_shift_code, write_byte_to_shift_register};
use arduino_uno::prelude::*;
use avr_hal_generic::ufmt::uwrite;
use common::protocol::{construct_request_packet, PacketType, RequestType};
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Configure UART
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );

    // Configure the shift I/O pins
    let mut data = pins.d11.into_output(&mut pins.ddr);
    let mut clock = pins.d9.into_output(&mut pins.ddr);
    let mut latch = pins.d12.into_output(&mut pins.ddr);
    let mut led = pins.d13.into_output(&mut pins.ddr);

    // Configure input
    let mut trigger = pins.d8;
    let mut is_triggered = false;

    loop {
        // Check for trigger input
        if trigger.is_high().unwrap() && !is_triggered {
            // Set button toggle latch
            is_triggered = true;

            // Trigger debug LED
            led.set_high().void_unwrap();

            // Send a request packet asking for a random number
            let packet = construct_request_packet(RequestType::RandomNumber);
            for byte in packet.iter() {
                serial.write_byte(*byte);
            }
            serial.flush();

            // We expect to get back a packet containing the response
            let mut incoming_packet = [0u8; 3];
            for i in 0..incoming_packet.len() {
                incoming_packet[i] = serial.read_byte();
            }

            // This will be filled with the received random number
            let mut rand_num = 0;

            // Ensure this is a valid packet
            if incoming_packet[0] == (PacketType::Response as u8) {
                rand_num = incoming_packet[2];
            } else {
                // Read failure
                is_triggered = false;
                continue;
            }

            // Get the shift code for the number
            let code = byte_to_shift_code(rand_num);

            // Write out to the display using a basic shift_out implementation
            write_byte_to_shift_register(code, &mut clock, &mut latch, &mut data);
        } else {
            // Trigger debug LED
            led.set_low().void_unwrap();
        }

        // Handle resetting the toggle latch
        if trigger.is_low().unwrap() {
            is_triggered = false;
        }
    }
}
