# TEJ4M Computer Interfacing Lab
Source and CAD for the TEJ4M computer interfacing lab


## The multi-processor communication protocol

### Request

| Data    | Size | Description                 |
| ------- | ---- | --------------------------- |
| `0x01`  | `u8` | Packet type (request)       |
| size    | `u8` | Packet size after this byte |
| request | `u8` | Type of data requested      |

### Response

| Data    | Size | Description                 |
| ------- | ---- | --------------------------- |
| `0x02`  | `u8` | Packet type (response)      |
| size    | `u8` | Packet size after this byte |
| payload | `u8` | Response payload            |