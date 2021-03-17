extern crate serial;

use clap::{App, Arg};
use serial::prelude::*;
use std::io::{BufReader, Read};
use std::time::Duration;

fn main() {
    let matches = App::new("Interface UI")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::with_name("port")
                .long("port")
                .takes_value(true)
                .help("Path to serial port")
                .required(true),
        )
        .get_matches();

    // Open the specified serial port
    let mut port = serial::open(matches.value_of("port").unwrap()).unwrap();

    // Configure the serial communication
    port.reconfigure(&|settings| {
        settings
            .set_baud_rate(serial::Baud9600)
            .expect("Failed to set serial baud");
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .expect("Failed to reconfigure serial port");
    port.set_timeout(Duration::from_millis(1000))
        .expect("Failed to configure serial timeout");

    // Set up input buffer
    let mut port = BufReader::with_capacity(6, port);

    loop {

        // Read serial data
        let mut data = vec![0u8; 6];
        port.read_exact(&mut data);

    }

}
