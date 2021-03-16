# Interface Protocol

This document is a rough outline of the coprocessor interface protocol. All data is transported over USB/FTDI Serial. Messages are sent as raw binary data

## Status Codes

| Code   | Status              |
| ------ | ------------------- |
| `0x00` | OK                  |
| `0x01` | Frame storage error |

## Init handshake

### 1. Host to coprocessor

| Data    | Size  | Description                  |
| ------- | ----- | ---------------------------- |
| `0x01`  | `u8`  | Packet type (host handshake) |
| size    | `u16` | Packet size in bytes         |
| version | `u8`  | Host-side protocol version   |

### 2. Coprocessor to host

| Data           | Size  | Description                                                         |
| -------------- | ----- | ------------------------------------------------------------------- |
| `0x02`         | `u8`  | Packet type (host handshake response)                               |
| size           | `u16` | Packet size in bytes                                                |
| version        | `u8`  | Coprocessor-side protocol version                                   |
| max frames     | `u8`  | The maximum number of animation frames supported by the coprocessor |
| display width  | `u8`  | Pixel width of the external display                                 |
| display height | `u8`  | Pixel height of the external display                                |

## Animation upload

### 1. Programming mode (H->C)

| Data   | Size  | Description                          |
| ------ | ----- | ------------------------------------ |
| `0x03` | `u8`  | Packet type (enter programming mode) |
| size   | `u16` | Packet size in bytes                 |
| count  | `u8`  | Number of animation frames to expect |

### 2. Mode Status (C->H)

| Data   | Size  | Description                  |
| ------ | ----- | ---------------------------- |
| `0x04` | `u8`  | Packet type (status)         |
| size   | `u16` | Packet size in bytes         |
| status | `u8`  | [Status code](#status-codes) |

### 3. Frame update (H->C)

This is repeated for `count` times (once per frame)

| Data     | Size     | Description                |
| -------- | -------- | -------------------------- |
| `0x05`   | `u8`     | Packet type (frame upload) |
| size     | `u16`    | Packet size in bytes       |
| raw data | `bool[]` | Raw frame binary data      |

### 4. Animation verify (C->H)

| Data        | Size  | Description                    |
| ----------- | ----- | ------------------------------ |
| `0x06`      | `u8`  | Packet type (verification)     |
| size        | `u16` | Packet size in bytes           |
| parity      | `u8`  | Parity of the entire animation |
| frame count | `u8`  | Number of full frames received |

## Playback

### Play (H->C)

| Data   | Size   | Description                           |
| ------ | ------ | ------------------------------------- |
| `0x07` | `u8`   | Packet type (play)                    |
| size   | `u16`  | Packet size in bytes                  |
| repeat | `bool` | Should this animation repeat forever? |

### Stop (H->C)

| Data   | Size  | Description          |
| ------ | ----- | -------------------- |
| `0x08` | `u8`  | Packet type (stop)   |
| `0x00` | `u16` | Packet size in bytes |

### Playback ended (C->H)

| Data   | Size  | Description                |
| ------ | ----- | -------------------------- |
| `0x09` | `u8`  | Packet type (playback end) |
| `0x00` | `u16` | Packet size in bytes       |