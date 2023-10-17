/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
    ::core::mem::size_of,
    byteorder::{LittleEndian, WriteBytesExt},
};

const SIZE_U32: usize = size_of::<u32>();

#[inline]
pub fn u32_to_array_u8(value: u32) -> [u8; SIZE_U32] {
    let mut buf = [0u8; SIZE_U32];
    buf.as_mut().write_u32::<LittleEndian>(value).expect("Unable to write");
    buf
}

pub fn array_u8_to_u64(bytes: &[u8]) -> u64 {
    (bytes[0] as u64)
        | (bytes[1] as u64) << 8
        | (bytes[2] as u64) << 16
        | (bytes[3] as u64) << 24
        | (bytes[4] as u64) << 32
        | (bytes[5] as u64) << 40
        | (bytes[6] as u64) << 48
        | (bytes[7] as u64) << 56
}

pub fn has_bits(number: u64, bits: u64) -> bool {
    (number & bits) == bits
}
