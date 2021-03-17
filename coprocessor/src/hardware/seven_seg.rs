/// Converts any byte to the corresponding shift register value (or 0 if invalid)
pub fn byte_to_shift_code(byte: u8) -> u8 {
    return match byte {
        0x00 => 252,
        0x01 => 96,
        0x02 => 218,
        0x03 => 242,
        0x04 => 102,
        0x05 => 182,
        0x06 => 190,
        0x07 => 224,
        0x08 => 254,
        0x09 => 246,
        0x0a => 238,
        0x0b => 62,
        0x0c => 156,
        0x0d => 122,
        0x0e => 158,
        0x0f => 142,
        _ => 0,
    };
}
