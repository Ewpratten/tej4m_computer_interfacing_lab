extern crate serial;

use clap::{App, Arg};
use common::protocol::{construct_response_packet, PacketType, RequestType};
use serial::prelude::*;
use std::io::{Read, Write};
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

    loop {
        // Read serial data
        let mut data = vec![0u8; 3];
        port.read_exact(&mut data);

        // Ensure the packet header is valid
        if data[0] == (PacketType::Request as u8) {
            println!("Received packet: {:?}", data);

            // Handle the request
            if data[2] == (RequestType::RandomNumber as u8) {
                // Fetch a random number
                let rand_num = rand::random::<u8>() % 0x0f;

                // Construct a packet
                let packet = construct_response_packet(rand_num);

                // Send back data
                port.write(&packet[..]);
                println!("Sent packet: {:?}", packet);
            }
        }
    }
}
