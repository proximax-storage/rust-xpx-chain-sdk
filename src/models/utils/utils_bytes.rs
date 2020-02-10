use ::core::mem;

use ::byteorder::{LittleEndian, WriteBytesExt};
use ::hex::encode;

pub(crate) fn u32_to_array_u8(value: u32) -> [u8; 4] {
    let mut buf = [0u8; mem::size_of::<u32>()];
    buf.as_mut().write_u32::<LittleEndian>(value).expect("Unable to write");
    return buf;
}

pub(crate) fn u64_to_array_u8(value: u64) -> [u8; 8] {
    let mut buf = [0u8; mem::size_of::<u64>()];
    buf.as_mut().write_u64::<LittleEndian>(value).expect("Unable to write");
    return buf;
}

pub(crate) fn array_u8_to_u32(bytes: [u8; 4]) -> u32 {
    (bytes[0] as u32) | (bytes[1] as u32) << 8 | (bytes[2] as u32) << 16 | (bytes[3] as u32) << 24
}

pub(crate) fn array_u8_to_u64(bytes: &[u8]) -> u64 {
    (bytes[0] as u64) |
        (bytes[1] as u64) << 8 |
        (bytes[2] as u64) << 16 |
        (bytes[3] as u64) << 24 |
        (bytes[4] as u64) << 32 |
        (bytes[5] as u64) << 40 |
        (bytes[6] as u64) << 48 |
        (bytes[7] as u64) << 56
}

pub(crate) fn vec_u8_to_hex(bytes: Vec<u8>) -> String {
    encode(bytes)
}
