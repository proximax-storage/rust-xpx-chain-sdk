
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionPayload {
    /// The transaction payload.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl TransactionPayload {
    pub fn new() -> TransactionPayload {
        TransactionPayload {
            payload: None,
        }
    }
}
