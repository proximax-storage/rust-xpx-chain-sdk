#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TransactionHashes {
    /// The array of transaction hashes.
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
}

impl From<Vec<&str>> for TransactionHashes {
    fn from(e: Vec<&str>) -> Self {
        let mut ids = TransactionHashes::default();
        for id in e {
            ids.hashes.push(id.trim().to_uppercase())
        }
        return ids;
    }
}
