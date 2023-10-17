/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::str::FromStr;

use hex::ToHex;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

// use crate::account::Address;
use crate::account::{decode_base32, encode_base32};

pub type GenerationHash = H256;
pub type TransactionHash = H256;

pub type Signer = H256;

construct_fixed_hash! {
	/// Symbol 256 bit hash type.
	// #[derive(Deserialize)]
	#[cfg_attr(any(test, feature = "proptest-support"), derive(proptest_derive::Arbitrary))]
	pub struct H256(32);
}

impl Serialize for H256 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.encode_hex_upper::<String>())
	}
}

impl<'de> Deserialize<'de> for H256 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		H256::from_str(string.as_ref()).map_err(|e| D::Error::custom(e))
	}
}

construct_fixed_hash! {
	/// Symbol 512 bit hash type.
	pub struct H512(64);
}

impl<'de> Deserialize<'de> for H512 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		H512::from_str(string.as_ref()).map_err(|e| D::Error::custom(e))
	}
}

construct_fixed_hash! {
	/// Symbol 200 bit hash type.
	#[cfg_attr(any(test, feature = "proptest-support"), derive(proptest_derive::Arbitrary))]
	pub struct H200(25);
}

impl H200 {
	pub fn as_base32(&self) -> String {
		encode_base32(self.as_bytes())
	}

	pub fn from_base32(data: &str) -> Self {
		let mut address = H200::zero();
		decode_base32(address.as_mut(), &data);
		address
	}
}

impl<'de> Deserialize<'de> for H200 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Ok(H200::from_base32(string.as_ref()))
	}
}

impl Serialize for H200 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.as_base32())
	}
}
