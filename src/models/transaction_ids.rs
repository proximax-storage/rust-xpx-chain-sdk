#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionIds {
    /// The array of transaction ids.
    #[serde(rename = "transactionIds", skip_serializing_if = "Option::is_none")]
    pub transaction_ids: Option<Vec<String>>,
}

impl TransactionIds {
    pub fn new() -> TransactionIds {
        TransactionIds {
            transaction_ids: None,
        }
    }
}


