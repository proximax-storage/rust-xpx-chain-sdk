/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::{
    models::{consts::PUBLIC_KEY_BYTES_SIZE, errors_const},
    utils::is_hex,
    Result,
};

use super::Address;

/// The `PublicAccount` account structure contains account's `Address` and public key.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicAccount {
    /// Retrieves the `Address` of this public account.
    pub address: Address,
    /// Retrieves the public key of this public account.
    pub public_key: String,
}

impl PublicAccount {
    /// Create a `PublicAccount` from a public key for the given `NetworkType`.
    pub fn from_public_key(
        public_key: &str,
        network_type: crate::models::network::NetworkType,
    ) -> Result<PublicAccount> {
        ensure!(
            !public_key.is_empty(),
            errors_const::ERR_INVALID_PUBLIC_KEY_LENGTH
        );

        ensure!(is_hex(public_key), errors_const::ERR_INVALID_KEY_HEX);

        ensure!(public_key.len() == 64, errors_const::ERR_INVALID_KEY_LENGTH);

        Ok(PublicAccount {
            address: Address::from_public_key(public_key, network_type)?,
            public_key: public_key.to_uppercase(),
        })
    }

    /// Verify a signature on a message with this `PublicAccount` public key.
    ///
    /// # Return
    ///
    /// Returns `Ok(())` if the signature is valid, and `Err` otherwise.
    pub fn verify_sign(&self, data: &str, signature: &str) -> Result<()> {
        ensure!(
            super::HASH512_LENGTH == (signature.len() / 2),
            errors_const::ERR_INVALID_SIGNATURE_LENGTH
        );

        ensure!(is_hex(signature), errors_const::ERR_INVALID_SIGNATURE_HEX);

        let sig_byte: Vec<u8> = hex::decode(signature)?;

        let pk_byte: Vec<u8> = hex::decode(&self.public_key)?;

        let pk = crypto::PublicKey::from_bytes(&pk_byte)?;

        let signature = crypto::Signature::from_bytes(&sig_byte)?;

        let verify = pk.verify(&data.as_bytes(), &signature);

        if verify.is_ok() {
            Ok(())
        } else {
            bail!(verify.unwrap_err())
        }
    }

    /// Convert this public key to a byte array.
    #[inline]
    pub fn to_bytes(&self) -> [u8; PUBLIC_KEY_BYTES_SIZE] {
        let mut array = [0; PUBLIC_KEY_BYTES_SIZE];
        let public_key_to_bytes = hex::decode(&self.public_key).unwrap();

        array.copy_from_slice(&public_key_to_bytes);

        array
    }

    pub fn public_key_string(&self) -> String {
        self.public_key.to_uppercase()
    }
}

impl core::fmt::Display for PublicAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
