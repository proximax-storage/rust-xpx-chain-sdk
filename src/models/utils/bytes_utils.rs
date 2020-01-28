use core::convert::TryFrom;

use futures::future::result;
use hex::encode;

pub(crate) fn u32_to_array_of_u8(value: i32) -> [u8; 4] {
    let mut buf = [0u8; 4];

    buf[0] = (value) as u8;
    buf[1] = (value >> 8) as u8;
    buf[2] = (value >> 16) as u8;
    buf[3] = (value >> 24) as u8;
    return buf;
}

pub(crate) fn array_of_u8_to_u32(bytes: [u8; 4]) -> i32 {
    let mut lower: i32 = (bytes[3] & 0xff) as i32;

    lower <<= 8;
    lower |= (bytes[2] & 0xff) as i32;
    lower <<= 8;
    lower |= (bytes[1] & 0xff) as i32;
    lower <<= 8;
    lower |= (bytes[0] & 0xff) as i32;
    return lower;
}

pub(crate) fn slice_to_array_of_u8(slice: &[u8]) -> [u8; 4] {
    let array = <&[u8; 4]>::try_from(slice).unwrap();
    return *array;
}

pub(crate) fn array_of_u8_to_hex_string(bytes: Vec<u8>) -> String {
    encode(bytes)
}
