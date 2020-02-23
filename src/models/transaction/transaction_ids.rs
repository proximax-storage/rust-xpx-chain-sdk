#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionIds {
    /// The array of transaction ids.
    #[serde(rename = "transactionIds")]
    pub transaction_ids: Vec<String>,
}

impl From<Vec<&str>> for TransactionIds {
    fn from(e: Vec<&str>) -> Self {
        let mut ids = TransactionIds::default();
        for id in e {
            ids.transaction_ids.push(id.to_uppercase())
        }
        return ids;
    }
}
