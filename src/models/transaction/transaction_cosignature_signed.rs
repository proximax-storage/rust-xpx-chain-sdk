use std::fmt;

use serde::{Serialize, Serializer};

use crate::models::{
    consts::SIGNATURE_SIZE,
    errors::ERR_INVALID_DATA_LENGTH,
    transaction::Hash,
    utils::{hex_to_vec_u8, is_hex}
};

pub struct Signature(pub(crate) [u8; SIGNATURE_SIZE]);

impl Signature {
    pub fn new(data: [u8; 64]) -> Self {
        Signature(data)
    }
    pub fn from_string(t: String) -> crate::Result<Self> {
        is_hex(&t);
        ensure!(
            !t.is_empty(),
            ERR_INVALID_DATA_LENGTH
         );

        Ok(Self::from_bytes(hex_to_vec_u8(&t).as_slice())?)
    }

    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        ensure!(
            bytes.len() == 64,
            ERR_INVALID_DATA_LENGTH
         );

        let mut bits: [u8; 64] = [0u8; 64];
        bits.copy_from_slice(&bytes[..64]);

        Ok(Self(bits))
    }

    /// Convert this Signature to a byte array.
    #[inline]
    pub fn to_bytes(&self) -> [u8; SIGNATURE_SIZE] {
        self.0
    }

    /// View this Signature as a byte array.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; SIGNATURE_SIZE] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        hex::encode_upper(&self.0[..])
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.0[..])
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CosignatureSignedTransaction {
    pub parent_hash: Hash,
    pub signature: Signature,
    pub signer: String,
}

impl CosignatureSignedTransaction {
    pub fn new(parent_hash: Hash, signature: Signature, signer: String) -> Self {
        Self { parent_hash, signature, signer }
    }
}

impl fmt::Display for CosignatureSignedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
