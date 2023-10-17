/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use anyhow::Result;
use crypto::{PUBLIC_KEY_LENGTH, PublicKey};
use hex::ToHex;

use crate::network::*;

use super::{Address, verify_signature};

/// The `PublicAccount` struct contains account's Sirius `Address` and public key.
///
#[derive(Default, Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
pub struct PublicAccount {
    /// the Sirius account's `Address`.
    pub address: Address,
    /// the Sirius account public key in `crypto PublicKey`.
    pub public_key: PublicKey,
}

impl PublicAccount {
    /// Account public key to hex String.
    ///
    pub fn public_key_to_hex(&self) -> String {
        self.public_key.encode_hex::<String>()
    }

    /// Account `NetworkType`.
    ///
    pub fn network_type(&self) -> NetworkType {
        self.address.network_type
    }

    /// Get the `Address` in an raw address string format.
    ///
    /// Example: TATNE7Q5BITMUTRRN6IB4I7FLSDRDWZA37JGO5Q
    pub fn address_as_string(&self) -> String {
        self.address.address_str()
    }

    /// Creates an Sirius `PublicAccount` from a given public_key string.
    ///
    /// # Inputs
    ///
    /// * `public_key`: representing the hex publick key (String or &str).
    ///
    /// * `network_type`: The `NetworkType` of Sybol Blockchain.
    ///
    /// # Example
    ///
    /// ```
    /// use xpx_chain_sdk::account::PublicAccount;
    /// use xpx_chain_sdk::network::NetworkType;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let public_key: &str = "2E834140FD66CF87B254A693A2C7862C819217B676D3943267156625E816EC6F";
    /// let public_account = PublicAccount::from_public_key(public_key, NetworkType::PublicTest).unwrap();
    /// # println!("{}", public_account);
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value is an Sirius `PublicAccount` or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn from_public_key<S: AsRef<str>>(
        public_key: S,
        network_type: NetworkType,
    ) -> Result<Self> {
        let address = Address::from_public_key(public_key.as_ref(), network_type)?;
        Ok(Self {
            address,
            public_key: PublicKey::from_bytes(&hex::decode(&public_key.as_ref()).unwrap()).unwrap(),
        })
    }

    /// Verify a signature.
    ///
    /// # Inputs
    ///
    /// * `data`: representing the data to verify.
    ///
    /// * `signature`: a `Signature` representing the signature to verify.
    ///
    /// # Returns
    ///
    /// A `Result` whose okay value True if the signature is valid or whose error value
    /// is an `Error` describing the error that occurred.
    pub fn verify_signature(&self, data: &str, signature: crypto::Signature) -> Result<()> {
        verify_signature(self.public_key, data, signature)
    }

    /// Create Builder object
    pub fn to_builder(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.public_key.as_bytes()
    }
}

impl fmt::Display for PublicAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
	use crypto::Signature;

	use crate::account::{Account, PublicAccount};
	use crate::network::NetworkType;

	const PUBLIC_KEY: &str = "b4f12e7c9f6946091e2cb8b6d3a12b50d17ccbbf646386ea27ce2946a7423dcf";

    #[test]
    fn test_should_create_from_public_key() {
        let public_account =
            PublicAccount::from_public_key(PUBLIC_KEY, NetworkType::PrivateTest).unwrap();
        assert_eq!(public_account.public_key_to_hex(), PUBLIC_KEY);
        assert_eq!(public_account.address_as_string(), "WARNASAS2BIAB6LMFA3FPMGBPGIJGK6IJHPRCU4F");
    }

    #[test]
    fn test_can_verify_a_signature() {
        let testing_account: Account = crate::account::account::tests::TESTING_ACCOUNT.clone();

        let signer_public_account = testing_account.public_account;
        let data =
            "ff60983e0c5d21d2fb83c67598d560f3cf0e28ae667b5616aaa58a059666cd8cf826b026243c92cf";
        let signature = testing_account.sign_data(data);
        assert!(signer_public_account.verify_signature(data, signature).is_ok());
    }

    #[test]
    fn test_return_false_if_wrong_public_key_provided() {
        let signer_public_account = PublicAccount::from_public_key(
            "12816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB509",
            NetworkType::PrivateTest,
        )
            .unwrap();

        let data = "I am so so so awesome as always";
        let signature = Signature::from_bytes(&hex::decode("B01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F3526FA508").unwrap()).unwrap();
        assert!(signer_public_account.verify_signature(data, signature).is_err());
    }

    #[test]
    fn test_return_false_if_data_is_not_corresponding_to_signature_provided() {
        let signer_public_account = PublicAccount::from_public_key(
            "22816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB508",
            NetworkType::PrivateTest,
        )
            .unwrap();

        let data = "I am awesome as always";
        let signature = Signature::from_bytes(&hex::decode("B01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F3526FA508").unwrap()).unwrap();
        assert!(signer_public_account.verify_signature(data, signature).is_err());
    }

    #[test]
    fn test_return_false_if_signature_is_not_corresponding_to_data_provided() {
        let signer_public_account = PublicAccount::from_public_key(
            "22816F825B4CACEA334723D51297D8582332D8B875A5829908AAE85831ABB508",
            NetworkType::PrivateTest,
        )
            .unwrap();

        let data = "I am so so so awesome as always";
        let signature = Signature::from_bytes(&hex::decode("A01DCA6484026C2ECDF3C822E64DEAAFC15EBCCE337EEE209C28513CB5351CDED8863A8E7B855CD471B55C91FAE611C548625C9A5916A555A24F72F3526FA509").unwrap()).unwrap();
        assert!(signer_public_account.verify_signature(data, signature).is_err());
    }
}
