extern crate math;
extern crate serial;

use clap::{App, Arg};
use raylib::prelude::*;
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

    // Set up graphics context
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Interfacing Demo")
        .build();

    // Joystick position data
    let mut joystick_vec = Vector2::new(0.0, 0.0);

    // 3D environment
    let camera = Camera3D::perspective(
        Vector3::new(0.0, 10.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    // Configure rendering
    rl.set_target_fps(60);

    // Main event loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Canvas clear
        d.clear_background(Color::WHITE);

        // Read serial data
        let mut data = vec![0u8; 6];
        port.read_exact(&mut data);

        // Ensure we have received a valid packet
        if data.len() >= 4 && data[0] == 0x01 {
            // Translate the sent joystick data into a 2d vector
            joystick_vec.x = ((data[2] as f32) - 128.0) / 128.0;
            joystick_vec.y = ((data[3] as f32) - 128.0) / 128.0;
        }

        // Render the 3D components
        {
            // 3D context
            let mut ctx = d.begin_mode3D(camera);

            // Render the ground plane
            ctx.draw_grid(10, 1.0);

            // Render interactive cube
            let cube_position = Vector3::new(joystick_vec.y * 5.0, 1.0, joystick_vec.x * -5.0);
            ctx.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
        }

        // Render HUD
        d.draw_text("Move the Joystick to interact", 10, 10, 20, Color::DARKGRAY);
    }
}
