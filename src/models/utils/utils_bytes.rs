use ::core::mem::size_of;

use ::byteorder::{LittleEndian, WriteBytesExt};
use ::hex::encode;
use hex::decode;

pub const SIZE_U32: usize = size_of::<u32>();

pub const SIZE_U64: usize = size_of::<u64>();

#[inline]
pub(crate) fn u32_to_array_u8(value: u32) -> [u8; SIZE_U32] {
    let mut buf = [0u8; SIZE_U32];
    buf.as_mut().write_u32::<LittleEndian>(value).expect("Unable to write");
    return buf;
}

pub(crate) fn array_u8_to_u32(bytes: [u8; SIZE_U32]) -> u32 {
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

pub(crate) fn hex_to_vec_u8(hex: &str) -> Vec<u8> {
    decode(hex).unwrap()
}

pub(crate) fn has_bits(number: u64, bits: u64) -> bool {
    (number & bits) == bits
}