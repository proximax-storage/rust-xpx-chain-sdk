// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use ::base32::Alphabet::RFC4648;

use crate::models::consts::{ADDRESS_DECODE_SIZE, ADDRESS_ENCODE_SIZE};
use crate::models::errors_const;
use crate::models::network::*;
use crate::Result;
use crate::utils::is_hex;

const PREFIX_MIJIN: char = 'M';
const PREFIX_MIJIN_TEST: char = 'S';
const PREFIX_PUBLIC: char = 'X';
const PREFIX_PUBLIC_TEST: char = 'V';
const PREFIX_PRIVATE: char = 'Z';
const PREFIX_PRIVATE_TEST: char = 'W';

const EMPTY_STRING: &str = "";
const REGEX_DASH: &str = "-";

/// The `Address` structure describes an address with its network.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The address in hexadecimal.
    pub address: String,
    pub network_type: NetworkType,
}

impl Address {
    /// Creates an `Address` from a given public_key string for the given `NetworkType`.
    pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(
            !public_key.is_empty(),
            errors_const::ERR_INVALID_PUBLIC_KEY_LENGTH
        );

        ensure!(is_hex(public_key), errors_const::ERR_INVALID_KEY_HEX);

        ensure!(public_key.len() == 64, errors_const::ERR_INVALID_KEY_LENGTH);

        let address = super::public_key_to_address(public_key, network_type)?;

        Ok(Self {
            address,
            network_type,
        })
    }

    /// Creates an `Address` from a given `raw_address` string.
    ///
    /// A raw address string looks like:
    /// VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU or VAWOEO-WTABXR-7O3ZAK-2XNA5G-IBNE6P-ZIXDAF-DWBU
    pub fn from_raw(raw_address: &str) -> Result<Self> {
        ensure!(!raw_address.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        let address = raw_address
            .trim()
            .to_uppercase()
            .replace(REGEX_DASH, EMPTY_STRING);
        ensure!(
            address.len() == ADDRESS_DECODE_SIZE,
            errors_const::ERR_INVALID_ADDRESSES_LEN
        );

        match address.chars().next().unwrap() {
            PREFIX_MIJIN => Ok(Self {
                address,
                network_type: MIJIN,
            }),
            PREFIX_MIJIN_TEST => Ok(Self {
                address,
                network_type: MIJIN_TEST,
            }),
            PREFIX_PUBLIC => Ok(Self {
                address,
                network_type: PUBLIC,
            }),
            PREFIX_PUBLIC_TEST => Ok(Self {
                address,
                network_type: PUBLIC_TEST,
            }),
            PREFIX_PRIVATE => Ok(Self {
                address,
                network_type: PRIVATE,
            }),
            PREFIX_PRIVATE_TEST => Ok(Self {
                address,
                network_type: PRIVATE_TEST,
            }),
            _ => bail!("Wrong address"),
        }
    }

    /// Create an `Address` from the given encoded address.
    pub fn from_encoded(encoded: &str) -> Result<Self> {
        ensure!(!encoded.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        ensure!(
            encoded.len() == ADDRESS_ENCODE_SIZE,
            errors_const::ERR_INVALID_ADDRESSES_LEN
        );

        ensure!(is_hex(encoded), errors_const::ERR_INVALID_ADDRESSES_HEX);

        let encoded_to_bytes = hex::decode(encoded)?;

        let address = base32::encode(RFC4648 { padding: true }, encoded_to_bytes.as_slice());

        Self::from_raw(address.as_str())
    }

    /// Converts an `Address` String into a more readable/pretty format.
    ///
    /// Before: VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU
    /// After: VAWOEO-WTABXR-7O3ZAK-2XNA5G-IBNE6P-ZIXDAF-DWBU
    pub fn prettify(&self) -> String {
        let mut res: String = String::new();

        for i in 0..6 {
            res += &self.address[i * 6..i * 6 + 6];
            res.push('-');
        }

        res += &self.address[&self.address.len() - 4..];
        return res;
    }

    pub fn is_empty(&self) -> bool {
        self.address.is_empty() && self.network_type == NOT_SUPPORTED_NET
    }

    #[inline]
    pub fn to_decode(&self) -> Vec<u8> {
        base32::decode(RFC4648 { padding: true }, &self.address).unwrap()
    }

    pub fn to_string(&self) -> String {
        self.address.to_uppercase()
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
