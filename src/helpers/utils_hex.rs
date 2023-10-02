/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use regex::Regex;

pub fn is_hex(input: &str) -> bool {
    if input == "" {
        return false;
    }

    let re = Regex::new(r"^[a-fA-F0-9]+$").unwrap();

    re.is_match(input)
}

pub fn hex_decode(data: &str) -> Vec<u8> {
    hex::decode(data)
        .map_err(|err| panic!("Failed to decode hex data {} : {}", data, err))
        .unwrap()
}

pub fn hex_encode(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_utf8(hex: &str) -> String {
    let decode = hex_decode(hex);
    String::from_utf8(decode).unwrap()
}

#[cfg(test)]
pub fn utf8_to_hex(txt: &str) -> String {
    use core::fmt::Write;
    let mut ret = String::with_capacity(2 * txt.len());
    for ch in txt.as_bytes() {
        write!(ret, "{:02x}", ch).expect("writing to string");
    }
    ret
}
