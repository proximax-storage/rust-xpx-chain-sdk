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
    pub hash: String,
}

impl SignedTransaction {
    pub(crate) fn new(entity_type: EntityTypeEnum, payload: String, hash: String) -> Self {
        SignedTransaction {
            payload: Some(payload),
            hash,
            entity_type,
        }
    }

    pub fn hash_to_bytes(&self) -> Vec<u8> {
        hex::decode(&self.hash ).unwrap()
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
