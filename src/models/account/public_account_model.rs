use crate::models::utils::is_hex;
use crate::Result;

use super::Address;

/// The `PublicAccount` account structure contains account's `Address` and public key.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicAccount {
    /// Retrieves the `Address` of this public account.
    pub address: Address,
    /// Retrieves the public key of this public account.
    pub public_key: String,
}

impl PublicAccount {
    /// Create a `PublicAccount` from a public key for the given `NetworkType`.
    pub fn from_public_key(public_key: &str, network_type: crate::models::network::NetworkType) -> Result<PublicAccount> {
        ensure!(
            !public_key.is_empty(),
            "public_key string is empty."
         );

        ensure!(
            is_hex(public_key),
            "Invalid hex public_key string."
            );

        ensure!(
            public_key.len() == 64,
            "Invalid len public_key string."
         );

        let _address = super::public_key_to_address(public_key, network_type);

        Ok(PublicAccount {
            address: Address::from_public_key(public_key, network_type),
            public_key: public_key.to_string(),
        })
    }

    /// Verify a signature on a message with this `PublicAccount` public key.
    ///
    /// # Return
    ///
    /// Returns `Ok(())` if the signature is valid, and `Err` otherwise.
    pub fn verify_sign(&self, data: &str, signature: &str) -> Result<bool> {
        ensure!(
            super::HASH512_LENGTH == (signature.len() / 2),
            "Signature length is incorrect"
            );

        ensure!(
            is_hex(signature),
            "Signature must be hexadecimal"
            );

        let sig_byte: Vec<u8> = hex::decode(signature)?;

        let pk_byte: Vec<u8> = hex::decode(&self.public_key)?;

        let pk = xpx_crypto::PublicKey::from_bytes(&pk_byte)?;

        let signature = xpx_crypto::Signature::from_bytes(&sig_byte)?;

        let verify = pk.verify(&data.as_bytes(), &signature);

        if verify.is_ok() {
            Ok(true)
        } else {
            Err(format_err!("{}", verify.unwrap_err()))
        }
    }

    pub fn to_array(&self) -> [u8; 32] {
        let mut array = [0; 32];
        let public_key_to_bytes = hex::decode(&self.public_key).unwrap();

        array.copy_from_slice(&public_key_to_bytes);

        return array;
    }
}

impl core::fmt::Display for PublicAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
