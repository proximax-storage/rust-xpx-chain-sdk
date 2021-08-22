/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use {
    ::std::ops::Deref,
    base32::Alphabet::RFC4648,
    serde::ser::SerializeStruct,
    serde::{Serialize, Serializer},
};

use crate::{
    helpers::{hex_decode, is_hex},
    models::{errors_const, network::*},
    Result,
};

use super::public_key_to_address;

pub(crate) const PREFIX_MIJIN: char = 'M';
pub(crate) const PREFIX_MIJIN_TEST: char = 'S';
pub(crate) const PREFIX_PUBLIC: char = 'X';
pub(crate) const PREFIX_PUBLIC_TEST: char = 'V';
pub(crate) const PREFIX_PRIVATE: char = 'Z';
pub(crate) const PREFIX_PRIVATE_TEST: char = 'W';

const EMPTY_STRING: &str = "";
const REGEX_DASH: &str = "-";

/// The [`Address`] structure describes an address with its [`NetworkType`].
#[derive(Default, Clone, PartialEq, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The address in bytes.
    address: [u8; Address::LENGTH],
    network_type: NetworkType,
}

impl Address {
    /// The length of the [`Address`] in bytes.
    pub const LENGTH: usize = 25;
    /// The length of the [`Address`] in bits.
    pub const LENGTH_IN_BITS: usize = Self::LENGTH * 8;
    /// The length of the [`Address`] in hex string.
    pub const LENGTH_IN_HEX: usize = Self::LENGTH * 2;
    /// The length of the [`Address`] in base32 string.
    pub const LENGTH_IN_BASE32: usize = 40;

    /// Creates an [`Address`] from a given public_key string for the given [`NetworkType`].
    pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Result<Self> {
        ensure!(
            !public_key.is_empty(),
            errors_const::ERR_INVALID_PUBLIC_KEY_LENGTH
        );

        ensure!(is_hex(public_key), errors_const::ERR_INVALID_KEY_HEX);

        ensure!(public_key.len() == 64, errors_const::ERR_INVALID_KEY_LENGTH);

        let address = public_key_to_address(public_key, network_type);

        Ok(Self {
            address,
            network_type,
        })
    }

    /// Creates an [`Address`] from a given `raw_address` string.
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
            address.len() == Self::LENGTH_IN_BASE32,
            errors_const::ERR_INVALID_ADDRESSES_LEN
        );

        let network_type = NetworkType::from(address.chars().next().unwrap());
        if network_type == NOT_SUPPORTED_NET {
            bail!(errors_const::ERR_WRONG_NETWORK_TYPE)
        }

        let address = Self::decode_from_base32(&address)?;

        Ok(Self {
            address,
            network_type,
        })
    }

    /// Create an [`Address`] from the given encoded address
    /// A raw address string hex looks like: A8EE6C6659D09B9CD25AAF1ED8796CE15000A19D31FD3BDE94.
    pub fn from_encoded(encoded: &str) -> Result<Self> {
        ensure!(!encoded.is_empty(), errors_const::ERR_EMPTY_ADDRESSES);

        ensure!(
            encoded.len() == Self::LENGTH_IN_HEX,
            errors_const::ERR_INVALID_ADDRESSES_LEN
        );

        ensure!(is_hex(encoded), errors_const::ERR_INVALID_ADDRESSES_HEX);

        let address = Self::decode_from_hex(encoded)?;

        Ok(Self {
            address,
            network_type: NetworkType::from(address[0]),
        })
    }

    /// Converts an [`Address`] String into a more readable/pretty format.
    ///
    /// Before: VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU
    /// After: VAWOEO-WTABXR-7O3ZAK-2XNA5G-IBNE6P-ZIXDAF-DWBU
    pub fn prettify(&self) -> String {
        let mut res: String = String::new();

        let address_string = &self.address_string();

        for i in 0..6 {
            res += &address_string[i * 6..i * 6 + 6];
            res.push('-');
        }

        res += &address_string[address_string.len() - 4..];
        res
    }

    #[inline]
    fn decode_from_base32(data: &str) -> Result<[u8; Self::LENGTH]> {
        let add_decode = base32::decode(RFC4648 { padding: true }, data);
        ensure!(add_decode.is_some(), errors_const::ERR_INVALID_ADDRESSES_BASE32);

        let mut bts: [u8; 25] = [0u8; 25];
        bts.copy_from_slice(&add_decode.unwrap());
        Ok(bts)
    }

    #[inline]
    fn decode_from_hex(data: &str) -> Result<[u8; Self::LENGTH]> {
        let add_decode = hex_decode(data);

        let mut bts: [u8; 25] = [0u8; 25];
        bts.copy_from_slice(&add_decode);
        Ok(bts)
    }

    #[inline]
    fn encode_as_base32(data: &[u8]) -> String {
        base32::encode(RFC4648 { padding: true }, data)
    }

    /// Get the address in an raw address string format.
    ///
    /// For example: VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU
    pub fn address_string(&self) -> String {
        Self::encode_as_base32(&self.address).to_uppercase()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.address
    }

    /// Get the address in an encoded format.
    ///
    /// For example: A8EE6C6659D09B9CD25AAF1ED8796CE15000A19D31FD3BDE94
    pub fn encode_as_hex(&self) -> String {
        hex::encode_upper(&self.address)
    }

    pub fn network_type(&self) -> NetworkType {
        self.network_type
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Address")
            .field("address", &self.address_string().to_lowercase())
            .field("network_type", &self.network_type)
            .finish()
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Address", 2)?;
        rgb.serialize_field("address", &self.address_string())?;
        rgb.serialize_field("network_type", &self.network_type)?;
        rgb.end()
    }
}

// Enable `Deref` coercion MosaicNonce.
impl Deref for Address {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.address[..]
    }
}
