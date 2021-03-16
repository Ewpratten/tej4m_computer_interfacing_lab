use clap::{App, Arg};
use raylib::prelude::*;
use std::time::Duration;
extern crate math;
extern crate serialport;

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
    let mut port = serialport::new(matches.value_of("port").unwrap(), 96_000)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open serial port");

    // Set up graphics context
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Interfacing Demo")
        .build();

    // Serial buffer
    let mut serial_buf: Vec<u8> = vec![0; 32];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Canvas clear
        d.clear_background(Color::WHITE);

        // Read serial data
        let result = port.read(&mut serial_buf);

        println!("{:?}", serial_buf);
    }
}
