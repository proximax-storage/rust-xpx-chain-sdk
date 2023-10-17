/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use {
	::std::{any::Any, fmt, str::FromStr},
	anyhow::Result,
	hex::ToHex,
	serde::Serialize,
};

use crate::{
	helpers::{hex_decode, is_hex, H200, H256},
	models::network::*,
};

use super::{
	decode_base32, encode_base32, is_valid_address, public_key_to_address, raw_prettify,
	UnresolvedAddress,
};

/// The `Address` struct describes an Sirius address with its network.
///
#[cfg_attr(any(test, feature = "proptest-support"), derive(proptest_derive::Arbitrary))]
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Address {
	/// The Sirius address in `H200`.
	pub address: H200,

	/// The Sirius network type.
	pub network_type: NetworkType,
}

impl Address {
	/// The length of the Sirius `Address` in base32 string.
	pub const LENGTH_IN_BASE32: usize = 40;

	/// Returns the size of a `Address` in bytes.
	pub const LENGTH_IN_DECODED: usize = std::mem::size_of::<H200>();

	pub const CHECKSUM_SIZE: usize = 4;

	/// Get the `Address` in an raw address string format.
	///
	/// For example: VA4FBXZTAF4F6WDPCVWXDB46Q4DCMCJVQBAG5PC2
	pub fn address_str(&self) -> String {
		encode_base32(self.address.as_bytes())
	}

	/// Converts `Address` String into a more readable/pretty format,
	/// a Sirius prettify address string looks like:
	///
	/// * Before: VA4FBXZTAF4F6WDPCVWXDB46Q4DCMCJVQBAG5PC2
	/// * After: VA4FBX-ZTAF4F-6WDPCV-WXDB46-Q4DCMC-JVQBAG-5PC2
	pub fn prettify(&self) -> String {
		raw_prettify(&self.address_str())
	}

	/// `Address` in the encoded format ex:
	///
	/// * Before: A83850DF3301785F586F156D71879E870626093580406EBC5A
	/// * After: VA4FBXZTAF4F6WDPCVWXDB46Q4DCMCJVQBAG5PC2
	pub fn address_encoded(&self) -> String {
		self.address.as_bytes().encode_hex_upper::<String>()
	}

	/// Creates an Sirius `Address` from a given public_key string.
	///
	/// # Inputs
	///
	/// * `public_key`: representing the hex publick key (String or &str).
	///
	/// * `network_type`: The `NetworkType` of Sirius Blockchain.
	///
	/// # Example
	///
	/// ```
	/// use xpx_chain_sdk::account::Address;
	/// use xpx_chain_sdk::network::NetworkType;
	///
	/// #
	/// # fn main() {
	/// #
	/// let public_key: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";
	/// let address = Address::from_public_key(public_key, NetworkType::PublicTest).unwrap();
	/// # println!("{}", address);
	/// # }
	/// ```
	///
	/// # Returns
	///
	/// A `Result` whose okay value is an Sirius `Address` or whose error value
	/// is an `Error` describing the error that occurred.
	pub fn from_public_key(public_key: &str, network_type: NetworkType) -> Result<Self> {
		ensure!(is_hex(public_key), "public_key it's not hex.");

		let public_key_hash =
			H256::from_str(public_key).map_err(|e| anyhow!("public_key {}", e))?;

		let address = public_key_to_address(public_key_hash, network_type);

		Ok(Self { address, network_type })
	}

	/// Creates an Sirius 'Address' from a given raw address string.
	///
	/// # Info
	///
	/// A Sirius raw address string looks like:
	/// * TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q or TATNE7-Q5BITM-UTRRN6-IB4I7F-LSDRDW-ZA37JG-O5Q.
	///
	/// # Inputs
	///
	/// * `raw_address`: an `S` representing address (String or &str).
	///
	/// # Example
	///
	/// ```
	/// use xpx_chain_sdk::account::Address;
	///
	/// #
	/// # fn main() {
	/// #
	/// let raw_address: &str = "VCSR3VXPNPOLGMFRAGEULG6IDIWLNRNYXOC3K5W2";
	/// let address = Address::from_raw(raw_address).unwrap();
	/// # println!("{}", address);
	/// # }
	/// ```
	///
	/// # Returns
	///
	/// A `Result` whose okay value is an Sirius `Address` or whose error value
	/// is an `Error` describing the error that occurred.
	pub fn from_raw<S: AsRef<str>>(raw_address: S) -> Result<Self> {
		let address_raw = raw_address.as_ref().trim().replace("-", "");
		ensure!(
			address_raw.len() == Self::LENGTH_IN_BASE32,
			"Address {} has to be {} characters long",
			address_raw,
			Self::LENGTH_IN_BASE32
		);

		let network_identifier = address_raw.to_uppercase().chars().next().unwrap();

		let network_type = NetworkType::try_from(network_identifier)?;

		let address = H200::from_base32(&address_raw);

		Ok(Self { address, network_type })
	}

	/// Create an Sirius `Address` from the given hex string address
	///
	/// A hex string address looks like: A8B270990F252CFFA99EACA63A98B74905AFBA214B22A521AC.
	///
	/// # Inputs
	///
	/// * `encoded`: an `S` representing address hex (String or &str).
	///
	/// # Example
	///
	/// ```
	/// use xpx_chain_sdk::account::Address;
	///
	/// #
	/// # fn main() {
	/// #
	/// let encoded: &str = "A8B270990F252CFFA99EACA63A98B74905AFBA214B22A521AC";
	/// let address = Address::from_encoded(encoded).unwrap();
	/// # println!("{}", address);
	/// # }
	/// ```
	///
	/// # Returns
	///
	/// A `Result` whose okay value is an Sirius `Address` or whose error value
	/// is an `Error` describing the error that occurred.
	pub fn from_encoded<S: AsRef<str>>(encoded: S) -> Result<Self> {
		ensure!(is_hex(encoded.as_ref()), "encoded address it's not hex.");

		let address =
			H200::from_str(encoded.as_ref()).map_err(|e| anyhow!("encoded address {}", e))?;

		Self::from_raw(encode_base32(address.as_bytes()))
	}

	/// Determines the validity of an raw address string.
	///
	/// # Inputs
	///
	/// * `raw_address`: The raw address string. Expected format VCZHBGIPEUWP7KM6VSTDVGFXJEC27ORBJMRKKINM.
	///
	/// # Returns
	///
	/// true if the raw address string is valid, false otherwise.
	///
	pub fn is_valid_raw_address(raw_address: &str) -> bool {
		// TODO: validate.
		// if !['F', 'V', 'J', 'Y', 'W'].contains(&raw_address.chars().last().unwrap()) {
		//     return false;
		// };
		let mut address = H200::zero();
		decode_base32(address.as_mut(), raw_address);

		is_valid_address(address.as_bytes(), Self::LENGTH_IN_DECODED, Self::CHECKSUM_SIZE)
	}

	/// Determines the validity of an encoded address string.
	/// # Inputs
	///
	/// * `encoded`: The encoded address string. Expected format: A8E4231EE10D552D9D40F0D78ECE1A4ED5A6DB4DE8F897FADC.
	/// # Returns
	///
	/// true if the encoded address string is valid, false otherwise.
	///
	pub fn is_valid_encoded_address(encoded: &str) -> bool {
		if !is_hex(encoded) {
			return false;
		}
		is_valid_address(&hex_decode(encoded), Self::LENGTH_IN_DECODED, Self::CHECKSUM_SIZE)
	}

	pub fn as_bytes(&self) -> &[u8] {
		self.address.as_bytes()
	}

	pub fn encode_as_hex(&self) -> String {
		self.address.encode_hex::<String>()
	}

	// /// Create Builder object
	// pub fn to_builder(&self) -> buffer::address_dto::AddressDto {
	//     buffer::address_dto::AddressDto(self.address.into())
	// }
}

#[typetag::serde]
impl UnresolvedAddress for Address {
	fn recipient_to_string(&self) -> String {
		self.address_str()
	}

	fn unresolved_address_to_bytes(&self, _network_type: NetworkType) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
	fn box_clone(&self) -> Box<dyn UnresolvedAddress + 'static> {
		Box::new((*self).clone())
	}
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
		self
	}
}

impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
	}
}

#[cfg(test)]
mod tests {
	// use crate::account::Account;
	use crate::account::Address;
	use crate::network::NetworkType;

	const PUBLIC_KEY: &str = "1c2a88e0dce41714fceff6e6adc5455d956708b1673109ded00db7934f88a4e2";

	#[test]
	fn test_should_print_the_address_in_pretty_format() {
		let address = Address::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
		assert_eq!(address.prettify(), "WCVE6E-C5EPQO-7PWXGZ-SNY4V2-SIHUNH-DKXDZM-XDMY");
	}

	#[test]
	fn test_should_create_given_public_key_with_network_type_private_test() {
		let address = Address::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
		assert_eq!(address.address_str(), "WCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDZMXDMY");
		assert_eq!(address.network_type, NetworkType::PrivateTest);
	}

	#[test]
	fn test_should_create_given_public_key_with_network_type_private() {
		let address = Address::from_public_key(PUBLIC_KEY, NetworkType::Private).unwrap();
		assert_eq!(address.address_str(), "ZCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDOYPDA6");
		assert_eq!(address.network_type, NetworkType::Private);
	}

	#[test]
	fn test_should_create_given_public_key_with_network_type_public_net() {
		let address = Address::from_public_key(PUBLIC_KEY, NetworkType::Public).unwrap();
		assert_eq!(address.address_str(), "XCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXA5KTMLO");
		assert_eq!(address.network_type, NetworkType::Public);
	}

	#[test]
	fn test_should_create_given_public_key_with_network_type_public_test_net() {
		let address = Address::from_public_key(PUBLIC_KEY, NetworkType::PublicTest).unwrap();
		assert_eq!(address.address_str(), "VCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXBNBWFCJ");
		assert_eq!(address.network_type, NetworkType::PublicTest);
	}

	#[test]
	fn test_should_create_given_wcve6ec5epqo7pwxgzsny4v2sihunhdkxdzmxdmy() {
		let address = Address::from_raw("WCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDZMXDMY").unwrap();
		assert_eq!(address.network_type, NetworkType::PrivateTest);
	}

	#[test]
	fn test_should_create_given_zcve6ec5epqo7pwxgzsny4v2sihunhdkxdoypda6() {
		let address = Address::from_raw("ZCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDOYPDA6").unwrap();
		assert_eq!(address.network_type, NetworkType::Private);
	}

	#[test]
	fn test_should_create_given_xcve6ec5epqo7pwxgzsny4v2sihunhdkxa5ktmlo() {
		let address = Address::from_raw("XCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXA5KTMLO").unwrap();
		assert_eq!(address.network_type, NetworkType::Public);
	}

	#[test]
	fn test_should_create_given_vcve6ec5epqo7pwxgzsny4v2sihunhdkxbnbwfcj() {
		let address = Address::from_raw("VCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXBNBWFCJ").unwrap();
		assert_eq!(address.network_type, NetworkType::PublicTest);
	}

	#[test]
	fn test_should_create_given_wcve6e_c5epqo_7pwxgz_sny4v2_sihunh_dkxdzm_xdmy() {
		let address = Address::from_raw("WCVE6E-C5EPQO-7PWXGZ-SNY4V2-SIHUNH-DKXDZM-XDMY").unwrap();
		assert_eq!(address.network_type, NetworkType::PrivateTest);
		assert_eq!(address.prettify(), "WCVE6E-C5EPQO-7PWXGZ-SNY4V2-SIHUNH-DKXDZM-XDMY");
	}

	#[test]
	#[should_panic(expected = "Address Network unsupported")]
	fn test_should_panic_when_the_address_contain_an_invalid_network_identifier() {
		Address::from_raw("ACVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXBNBWFCJ").unwrap();
	}

	#[test]
	#[should_panic(
		expected = "Address ZCTVW234AQ4TZIDZENGNOZXPRPSDRSFRF has to be 40 characters long"
	)]
	fn test_should_panic_when_the_address_is_not_valid_in_length() {
		Address::from_raw("ZCTVW234AQ4TZIDZENGNOZXPRPSDRSFRF").unwrap();
	}

	#[test]
	fn test_should_turn_a_lowercase_address_to_uppercase() {
		let address = Address::from_raw("wcve6ec5epqo7pwxgzsny4v2sihunhdkxdzmxdmy").unwrap();
		assert_eq!(address.address_str(), "WCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDZMXDMY");
	}

	#[test]
	fn test_should_equal_addresses() {
		let address = Address::from_raw("WBUWLWR2BPKT7GK4FDRDB2SAZNJ5PQL6O425DQ2G").unwrap();
		let compare_address =
			Address::from_raw("WBUWLWR2BPKT7GK4FDRDB2SAZNJ5PQL6O425DQ2G").unwrap();

		assert_eq!(address, compare_address);
	}

	#[test]
	fn test_should_not_equal_addresses() {
		let address = Address::from_raw("WBUWLWR2BPKT7GK4FDRDB2SAZNJ5PQL6O425DQ2G").unwrap();
		let compare_address =
			Address::from_raw("WCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDZMXDMY").unwrap();

		assert_ne!(address, compare_address);
	}

	#[test]
	fn test_should_creates_from_an_encoded_value() {
		let encoded = "908E2C873E8552039933562AFB74A193B48BDD300BEBDB9309";
		let address = Address::from_encoded(encoded).unwrap();
		assert_eq!(address.address_encoded(), encoded);
	}

	#[cfg(test)]
	mod tests_valid_raw_address {
		use crate::account::Account;

		use super::*;

		#[test]
		fn test_returns_true_for_valid_address_when_generated() {
			assert!(Address::is_valid_raw_address(
				&Account::random(NetworkType::PrivateTest).address_str()
			));
			assert!(Address::is_valid_raw_address(
				&Account::random(NetworkType::Public).address_str()
			));
			assert!(Address::is_valid_raw_address(
				&Account::random(NetworkType::Private).address_str()
			));
			assert!(Address::is_valid_raw_address(
				&Account::random(NetworkType::PublicTest).address_str()
			));
		}

		#[test]
		fn test_returns_true_for_valid_address() {
			let raw_address = "WCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDZMXDMY";
			assert!(Address::is_valid_raw_address(raw_address));
		}

		#[test]
		fn test_returns_false_for_address_with_invalid_checksum() {
			let raw_address = "SATNE7Q5BITMUTRRN6YB4I7FLSDRDWZA34I2XPMQ";
			assert_eq!(Address::is_valid_raw_address(raw_address), false);
		}

		#[test]
		fn test_returns_false_for_address_with_invalid_hash() {
			let raw_address = "SATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA34I2XPQQ";
			assert_eq!(Address::is_valid_raw_address(raw_address), false);
		}

		#[test]
		fn test_returns_false_for_address_with_invalid_prefix() {
			let raw_address = "AATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA34I2XPMQ";
			assert_eq!(Address::is_valid_raw_address(raw_address), false);
		}

		#[test]
		fn test_returns_true_if_last_char_in_a_or_i_or_q_or_y() {
			const RAW_ADDRESS: [&str; 4] = [
				"XCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXA5KTMLO",
				"VCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXBNBWFCJ",
				"ZCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDOYPDA6",
				"WCVE6EC5EPQO7PWXGZSNY4V2SIHUNHDKXDZMXDMY",
			];

			for address in &RAW_ADDRESS {
				assert!(Address::is_valid_raw_address(address));
			}
		}
	}

	#[cfg(test)]
	mod tests_valid_encoded_address {
		use super::*;
		use crate::account::Account;
		use proptest::prelude::*;
		use proptest::proptest;

		#[test]
		fn test_returns_true_for_valid_address_when_generated() {
			assert!(Address::is_valid_encoded_address(
				Account::random(NetworkType::PrivateTest)
					.public_account
					.address
					.address_encoded()
					.as_str()
			));
			assert!(Address::is_valid_encoded_address(
				Account::random(NetworkType::Public)
					.public_account
					.address
					.address_encoded()
					.as_str()
			));
			assert!(Address::is_valid_encoded_address(
				Account::random(NetworkType::Private)
					.public_account
					.address
					.address_encoded()
					.as_str()
			));
			assert!(Address::is_valid_encoded_address(
				Account::random(NetworkType::PublicTest)
					.public_account
					.address
					.address_encoded()
					.as_str()
			));
		}

		#[test]
		fn test_returns_true_for_valid_encoded_address() {
			let encoded = "c8dd8dfd359efa02adf0a2add1e2e57ea77ec4c35628d5910e";
			assert!(Address::is_valid_encoded_address(encoded));
		}

		#[test]
		fn test_returns_false_for_invalid_hex_encoded_address() {
			let encoded = "Z823BB7C3C089D996585466380EDBDC19D4959184893E38C";
			assert_eq!(Address::is_valid_encoded_address(encoded), false);
		}

		#[test]
		fn test_returns_false_for_invalid_encoded_address() {
			let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38D";
			assert_eq!(Address::is_valid_encoded_address(encoded), false);
		}

		#[test]
		fn test_returns_false_for_encoded_address_with_wrong_length() {
			let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38CEE";
			assert_eq!(Address::is_valid_encoded_address(encoded), false);
		}

		#[test]
		fn test_adding_leading_or_trailing_white_space_invalidates_encoded_address() {
			let encoded = "6823BB7C3C089D996585466380EDBDC19D4959184893E38C";
			assert_eq!(Address::is_valid_encoded_address(&format!(" \t {}", encoded)), false);
			assert_eq!(Address::is_valid_encoded_address(&format!("{} \t ", encoded)), false);
			assert_eq!(Address::is_valid_encoded_address(&format!(" \t {} \t ", encoded)), false);
		}

		proptest! {
			#![proptest_config(ProptestConfig::with_cases(1))]
			#[test]
			fn test_is_valid_encoded_address(encoded in any::<Address>()) {
					prop_assert_eq!(Address::is_valid_encoded_address(encoded.address_encoded().as_str()), true);
			}
		}
	}
}
