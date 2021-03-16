use clap::{App, Arg};
use raylib::prelude::*;
use std::time::Duration;
extern crate math;
extern crate serialport;
use std::io::BufRead;
use std::io::BufReader;

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
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open serial port");

    // Set up graphics context
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Interfacing Demo")
        .build();

    // Serial buffer
    let buffer = BufReader::new(port);

    // Joystick position data
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    for line in buffer.lines() {
        println!("{:?}", line);
    }

    // Main event loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Canvas clear
        d.clear_background(Color::WHITE);

        // // Read serial data
        // let result = buffer.read();
        // if result.is_ok() {

        //     let len = result.unwrap();

        //     println!("{:?}", len);

        //     // // Parse the incoming packet
        //     // if serial_buf[0] == 0x01 {
        //     //     x = ((serial_buf[2] as f32) - 128.0) / 128.0;
        //     //     y = ((serial_buf[3] as f32) - 128.0) / 128.0;
        //     //     println!("{} - {}", x, y);
        //     // }
        // }
    }
}
