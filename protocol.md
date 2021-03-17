# Host/Coprocessor interface protocol

Communication from the coprocessor is one-way and unchecked. Similar to UDP, but over USB/UART.

## Joystick position updates

These messages are sent to the host any time a change in state is detected on the joystick, or once per second.

| Data   | Size | Description                                 |
| ------ | ---- | ------------------------------------------- |
| `0x01` | `u8` | Packet ID                                   |
| size   | `u8` | Packet size (excluding the first two bytes) |
| x      | `u8` | X rotation as a mapped byte                 |
| y      | `u8` | Y rotation as a mapped byte                 |