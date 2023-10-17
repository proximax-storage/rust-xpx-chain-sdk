/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

//! This code is a Rust implementation of a custom data type called `NetworkType` which represents a type of network
//! for a Sirius blockchain. It has several variant options such as `PublicTest`, `Public`, `Private`, `PrivateTest`, `MijinTest`,
//! and `Mijin`.
//!
//! The `NetworkType` enum implements several functions including:
//!
//! The `value` return the numeric value of the `NetworkType`.
//! The `prefix` return a prefix character for the `NetworkType`.
//! The `to_bytes` return the `NetworkType` as a byte array.
//! The `from_transaction_version` return the `NetworkType` from a transaction version.
//!
//! The `TryFrom` trait is also implemented for the `NetworkType` enum to allow for converting a u8 or char value into a
//! `NetworkType`. If the conversion fails, an error message is returned.
//!
//! Finally, the Display and Debug traits are implemented for `NetworkType` to allow for custom formatting of its value
//! when displayed.
//!

use std::{convert::TryFrom, fmt};

#[allow(non_camel_case_types)]
#[cfg_attr(any(test, feature = "proptest-support"), derive(proptest_derive::Arbitrary))]
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash, strum::Display)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum NetworkType {
	/// The Public test net network identifier.
	/// Decimal value = 168
	/// Hex value = 0xa8
	///
	PublicTest = 0xa8,

	/// The public main net network identifier.
	/// Decimal value = 184
	/// Hex value = 0xb8
	///
	Public = 0xb8,

	/// The Private net network identifier.
	/// Decimal value = 200
	/// Hex value = 0xc8
	///
	Private = 0xc8,

	/// The Private test net network identifier.
	/// Decimal value = 176
	/// Hex value = 0xb0
	///
	PrivateTest = 0xb0,

	/// Mijin private test network identifier.
	/// Decimal value = 144
	/// Hex value = 0x90
	///
	MijinTest = 0x90,

	/// Mijin private network identifier.
	/// Decimal value = 96
	/// Hex value = 0x60
	///
	Mijin = 0x60,
}

impl NetworkType {
	pub const PREFIX_PUBLIC_TEST_NET: char = 'V';
	pub const PREFIX_PUBLIC_NET: char = 'X';
	pub const PREFIX_PRIVATE_TEST: char = 'W';
	pub const PREFIX_PRIVATE: char = 'Z';
	pub const PREFIX_MIJIN_TEST: char = 'S';
	pub const PREFIX_MIJIN: char = 'M';

	pub(crate) const UNKNOWN_NETWORK_TYPE: &'static str = "Address Network unsupported";

	pub fn value(self) -> u8 {
		self as u8
	}

	pub fn prefix(&self) -> char {
		use NetworkType::*;
		match *self {
			PublicTest => Self::PREFIX_PUBLIC_TEST_NET,
			Public => Self::PREFIX_PUBLIC_NET,
			PrivateTest => Self::PREFIX_PRIVATE_TEST,
			Private => Self::PREFIX_PRIVATE,
			MijinTest => Self::PREFIX_MIJIN_TEST,
			Mijin => Self::PREFIX_MIJIN,
		}
	}

	pub fn to_bytes(&self) -> [u8; 1] {
		self.value().to_le_bytes()
	}

	pub fn from_transaction_version(version: u32) -> anyhow::Result<NetworkType> {
		NetworkType::try_from((version >> 24) as u8)
	}
}
//
// impl fmt::Display for NetworkType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", &self)
//     }
// }

impl fmt::Debug for NetworkType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self.value())
	}
}

/// Returns a `NetworkType` for the given u8 value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl TryFrom<u8> for NetworkType {
	type Error = anyhow::Error;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		use NetworkType::*;
		match v {
			x if x == Public as u8 => Ok(Public),
			x if x == PublicTest as u8 => Ok(PublicTest),
			x if x == Private as u8 => Ok(Private),
			x if x == PrivateTest as u8 => Ok(PrivateTest),
			x if x == Mijin as u8 => Ok(Mijin),
			x if x == MijinTest as u8 => Ok(MijinTest),
			_ => Err(anyhow::anyhow!(Self::UNKNOWN_NETWORK_TYPE)),
		}
	}
}

/// Returns a 'NetworkType' for the given char value.
///
/// Throws an Err UNKNOWN_NETWORK_TYPE when the type is unknown.
impl TryFrom<char> for NetworkType {
	type Error = anyhow::Error;

	fn try_from(ch: char) -> Result<Self, Self::Error> {
		use NetworkType::*;
		match ch {
			Self::PREFIX_PUBLIC_TEST_NET => Ok(PublicTest),
			Self::PREFIX_PUBLIC_NET => Ok(Public),
			Self::PREFIX_PRIVATE_TEST => Ok(PrivateTest),
			Self::PREFIX_PRIVATE => Ok(Private),
			Self::PREFIX_MIJIN_TEST => Ok(MijinTest),
			Self::PREFIX_MIJIN => Ok(Mijin),
			_ => Err(anyhow::anyhow!(Self::UNKNOWN_NETWORK_TYPE)),
		}
	}
}

/// Creates `NetworkType` with the default parameters.
///
impl Default for NetworkType {
	fn default() -> Self {
		NetworkType::PublicTest
	}
}

impl From<NetworkType> for u8 {
	fn from(value: NetworkType) -> Self {
		value as u8
	}
}

#[cfg(test)]
mod tests {
	use crate::network::NetworkType;

	#[test]
	fn test_main_net_is_0xb8() {
		assert_eq!(NetworkType::Public as u8, 0xb8);
		assert_eq!(NetworkType::Public as u8, 184);
	}

	#[test]
	fn test_test_net_is_0xa8() {
		assert_eq!(NetworkType::PublicTest as u8, 0xa8);
		assert_eq!(NetworkType::PublicTest as u8, 168);
	}

	#[test]
	fn test_private_test_is_0xb0() {
		assert_eq!(NetworkType::PrivateTest as u8, 0xb0);
		assert_eq!(NetworkType::PrivateTest as u8, 176);
	}

	#[test]
	fn test_private_is_0xc8() {
		assert_eq!(NetworkType::Private as u8, 0xc8);
		assert_eq!(NetworkType::Private as u8, 200);
	}

	#[test]
	fn test_mijin_is_0x60() {
		assert_eq!(NetworkType::Mijin as u8, 0x60);
		assert_eq!(NetworkType::Mijin as u8, 96);
	}

	#[test]
	fn test_mijin_test_is_0x90() {
		assert_eq!(NetworkType::MijinTest as u8, 0x90);
		assert_eq!(NetworkType::MijinTest as u8, 144);
	}
}
