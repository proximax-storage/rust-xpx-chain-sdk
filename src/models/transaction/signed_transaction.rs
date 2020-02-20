use crate::models::network::NetworkType;

use super::TransactionType;

/// Used to transfer the transaction data and the signature to a nem server in order to
/// initiate and broadcast a transaction.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SignedTransaction {
    /// The transaction type.
    pub entity_type: TransactionType,

    /// The serialized transaction data.
    pub payload: String,

    /// The transaction hash.
    pub hash: String,
}

impl SignedTransaction {
    pub(crate) fn new(entity_type: TransactionType, payload: String, hash: String) -> Self {
        SignedTransaction {
            payload,
            hash,
            entity_type,
        }
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
