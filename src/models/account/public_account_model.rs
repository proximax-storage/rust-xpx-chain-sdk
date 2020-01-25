use models::{InternalError, ModelError};
use models::account::Address;

/// The `PublicAccount` account structure contains account's `Address` and public key.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicAccount {
    /// Retrieves the `Address` of this public account.
    #[serde(rename = "address")]
    pub address: Address,
    /// Retrieves the public key of this public account.
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

impl PublicAccount {
    /// Create a `PublicAccount` from a public key for the given `NetworkType`.
    pub fn from_public_key(public_key: &str, network_type: crate::models::network::NetworkType) -> PublicAccount {
        let _address = ::models::account::public_key_to_address(public_key, network_type);

        PublicAccount {
            address: Address::from_public_key(public_key, network_type),
            public_key: public_key.to_string(),
        }
    }

    /// Verify a signature on a message with this `PublicAccount` public key.
    ///
    /// # Return
    ///
    /// Returns `Ok(())` if the signature is valid, and `Err` otherwise.
    pub fn verify_sign(&self, data: &str, signature: &str) -> Result<(), ModelError> {
        if ::models::account::HASH512_LENGTH != (signature.len() / 2) {
            return Err(ModelError(InternalError::InvalidSignatureLenError));
        };

        if !::models::account::is_hex(signature) {
            return Err(ModelError(InternalError::InvalidSignatureHexError));
        };

        let sig_byte: Vec<u8> = hex::decode(signature).unwrap();

        let pk_byte: Vec<u8> = hex::decode(&self.public_key).unwrap();

        let pk = xpx_crypto::PublicKey::from_bytes(&pk_byte);

        if pk.is_err() {
            return Err(ModelError::from(&pk.unwrap_err()));
        }

        let signature = xpx_crypto::Signature::from_bytes(&sig_byte);

        if signature.is_err() {
            return Err(ModelError::from(&signature.unwrap_err()));
        }

        let verify = pk.unwrap().verify(&data.as_bytes(), &signature.unwrap());

        if verify.is_ok() {
            Ok(())
        } else {
            Err(ModelError::from(&verify.unwrap_err()))
        }
    }
}
