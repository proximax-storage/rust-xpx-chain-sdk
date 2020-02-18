use super::TransactionType;
use crate::models::network::NetworkType;

/// Used to transfer the transaction data and the signature to a nem server in order to
/// initiate and broadcast a transaction.
#[derive(Debug, PartialEq, Serialize)]
pub struct  SignedTransaction {
    /// The serialized transaction data.
    pub payload: String,

    /// The transaction hash.
    pub hash: String,

    /// The transaction signer.
    pub signer: String,

    /// The transaction type.
    pub _type: TransactionType,

    /// The signer network type.
    pub network_type: NetworkType
}

impl core::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
