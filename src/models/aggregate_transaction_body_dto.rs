#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateTransactionBodyDto {
    /// An array of transaction cosignatures.
    #[serde(rename = "cosignatures")]
    pub cosignatures: Vec<crate::models::CosignatureDto>,
    /// The array of transactions initiated by different accounts.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::EmbeddedTransactionInfoDto>,
}

impl AggregateTransactionBodyDto {
    pub fn new(cosignatures: Vec<crate::models::CosignatureDto>, transactions: Vec<crate::models::EmbeddedTransactionInfoDto>) -> AggregateTransactionBodyDto {
        AggregateTransactionBodyDto {
            cosignatures,
            transactions,
        }
    }
}


