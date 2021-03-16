import serial
import argparse
import os
import sys
import logging
from rich.logging import RichHandler
from ctypes import byref

# Handle loading a custom RayLib
os.environ["RAYLIB_BIN_PATH"] = "./raylib/release/libs/linux"
import raylibpy as rl

FORMAT = "%(message)s"
logging.basicConfig(
    level="NOTSET", format=FORMAT, datefmt="[%X]", handlers=[RichHandler()]
)


def main() -> int:

    # Handle CLI args
    ap = argparse.ArgumentParser()
    ap.add_argument("port", help="Serial port to communicate with coprocessor")
    args = ap.parse_args()

    # Init serial communication
    logging.info(f"Opening serial connection to {args.port}")
    serial_comms = serial.Serial(args.port)

    # Init graphics
    rl.init_window(800, 600, "TEJ4M Computer Interfacing")

    # Set up 3D environment
    camera = rl.Camera(
        rl.Vector3(4.0, 2.0, 4.0),
        rl.Vector3(0.0, 0.0, 0.0),
        rl.Vector3(0.0, 1.0, 0.0),
        45.0,
        rl.CAMERA_PERSPECTIVE
    )

    cube = rl.Vector3(0.0, 0.0, 0.0)

    # Configure 3D Rendering
    rl.set_camera_mode(camera, rl.CAMERA_FREE)
    rl.set_target_fps(60)

    while not rl.window_should_close():
        rl.update_camera(byref(camera))
        if rl.is_key_down(rl.KEY_Z):
            camera.target = rl.Vector3(0.0, 0.0, 0.0)

        rl.begin_drawing()

        rl.clear_background(rl.RAYWHITE)

        rl.begin_mode3d(camera)

        rl.draw_cube(cube_position, 2.0, 2.0, 2.0, rl.RED)
        rl.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, rl.MAROON)

        rl.draw_grid(10, 1.0)

        rl.end_mode3d()

        rl.draw_rectangle(10, 10, 320, 133, rl.fade(rl.SKYBLUE, 0.5))
        rl.draw_rectangle_lines(10, 10, 320, 133, rl.BLUE)
        
        rl.draw_text("Free camera default controls:", 20, 20, 10, rl.BLACK)
        rl.draw_text("- Mouse Wheel to Zoom in-out", 40, 40, 10, rl.DARKGRAY)
        rl.draw_text("- Mouse Wheel Pressed to Pan", 40, 60, 10, rl.DARKGRAY)
        rl.draw_text("- Alt + Mouse Wheel Pressed to Rotate", 40, 80, 10, rl.DARKGRAY)
        rl.draw_text("- Alt + Ctrl + Mouse Wheel Pressed for Smooth Zoom", 40, 100, 10, rl.DARKGRAY)
        rl.draw_text("- Z to Zoom to (0, 0, 0)", 40, 120, 10, rl.DARKGRAY)

        rl.end_drawing()

    # Clean up
    logging.info("Cleaning up and shutting down")
    rl.close_window()
    serial_comms.close()

    return 0


if __name__ == "__main__":
    sys.exit(main())
