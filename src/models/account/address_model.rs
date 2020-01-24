use base32::Alphabet::RFC4648;

use models::{InternalError, ModelError};
use models::network::*;

const PREFIX_MIJIN: char = 'M';
const PREFIX_MIJIN_TEST: char = 'S';
const PREFIX_PUBLIC: char = 'X';
const PREFIX_PUBLIC_TEST: char = 'V';
const PREFIX_PRIVATE: char = 'Z';
const PREFIX_PRIVATE_TEST: char = 'W';

const EMPTY_STRING: &str = "";
const REGEX_DASH: &str = "-";

/// The `Address` structure describes an address with its network.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// The address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "networkType")]
    pub network_type: crate::models::network::NetworkType,
}

impl Address {
    /// Creates an `Address` from a given public_key string for the given `NetworkType`.
    pub fn from_public_key(public_key: &str, network_type: crate::models::network::NetworkType) -> Address {
        let _address = ::models::account::public_key_to_address(public_key, network_type.0);

        Address {
            address: _address,
            network_type,
        }
    }

    /// Creates an `Address` from a given `raw_address` string.
    ///
    /// A raw address string looks like:
    /// VAWOEOWTABXR7O3ZAK2XNA5GIBNE6PZIXDAFDWBU or VAWOEO-WTABXR-7O3ZAK-2XNA5G-IBNE6P-ZIXDAF-DWBU
    pub fn from_raw(raw_address: &str) -> Result<Address, ModelError> {
        let _address = raw_address.trim().to_uppercase().replace(REGEX_DASH, EMPTY_STRING);

        match _address.chars().next().unwrap() {
            PREFIX_MIJIN => Ok(Address { address: _address, network_type: MIJIN }),
            PREFIX_MIJIN_TEST => Ok(Address { address: _address, network_type: MIJIN_TEST }),
            PREFIX_PUBLIC => Ok(Address { address: _address, network_type: PUBLIC }),
            PREFIX_PUBLIC_TEST => Ok(Address { address: _address, network_type: PUBLIC_TEST }),
            PREFIX_PRIVATE => Ok(Address { address: _address, network_type: PRIVATE }),
            PREFIX_PRIVATE_TEST => Ok(Address { address: _address, network_type: PRIVATE_TEST }),
            _ => Err(ModelError(InternalError::InvalidAddressError))
        }
    }

    /// Create an `Address` from the given encoded address.
    pub fn from_encoded(encoded: &str) -> Result<Address, ModelError> {
        let _encoded_to_bytes = hex::decode(encoded).unwrap();

        let _address = base32::encode(RFC4648 { padding: true }, _encoded_to_bytes.as_slice());

        self::Address::from_raw(_address.as_str())
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
        };

        res += &self.address[&self.address.len() - 4..];
        return res;
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "networkType: {}, address: {}", self.network_type, self.address)
    }
}
