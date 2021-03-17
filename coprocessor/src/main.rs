#![no_std]
#![no_main]

mod hardware;

use crate::hardware::seven_seg::byte_to_shift_code;
use arduino_uno::prelude::*;
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

            // Fetch a random number
            let rand_num = 6;

            // Get the shift code for the number
            let code = byte_to_shift_code(rand_num);

            // Write out to the display using a basic shift_out implementation
            latch.set_low().void_unwrap();
            for i in 0..8 {
                if (code & (1 << (7 - i))) > 0 {
                    data.set_high().void_unwrap();
                } else {
                    data.set_low().void_unwrap();
                }

                // Toggle the clock
                clock.set_high().void_unwrap();
                arduino_uno::delay_ms(10);
                clock.set_low().void_unwrap();
            }
            latch.set_high().void_unwrap();
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
