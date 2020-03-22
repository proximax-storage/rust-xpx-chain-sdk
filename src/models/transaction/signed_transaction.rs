use crate::models::transaction::Hash;

use super::EntityTypeEnum;

/// Used to transfer the transaction data and the signature to a nem server in order to
/// initiate and broadcast a transaction.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SignedTransaction {
    /// The transaction type.
    pub entity_type: EntityTypeEnum,

    /// The serialized transaction data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    /// The transaction hash.
    pub hash: Hash,
}

impl SignedTransaction {
    pub(crate) fn new(entity_type: EntityTypeEnum, payload: String, hash: Hash) -> Self {
        SignedTransaction {
            payload: Some(payload),
            hash,
            entity_type,
        }
    }

    pub(crate) fn hash_to_bytes(&self) -> Vec<u8> {
        hex::decode(&self.hash).unwrap()
    }

    pub fn get_payload(&self) -> String {
        match self.payload.to_owned() {
            Some(payload) => payload,
            _ => "".to_string()
        }
    }

    pub fn get_type(&self) -> EntityTypeEnum {
        self.entity_type
    }

    pub fn get_hash(&self) -> Hash {
        self.hash.to_owned()
    }

    pub fn type_to_string(&self) -> String {
        self.entity_type.to_string()
    }
}

impl core::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
