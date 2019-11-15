#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionHashes {
    /// The array of transaction hashes.
    #[serde(rename = "hashes", skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<String>>,
}

impl TransactionHashes {
    pub fn new() -> TransactionHashes {
        TransactionHashes {
            hashes: None,
        }
    }
}


