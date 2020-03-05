/// SourceDto : The transaction that triggered the receipt.
#[derive(Serialize, Deserialize)]
pub struct SourceDto {
    /// The transaction index within the block.
    #[serde(rename = "primaryId")]
    pub primary_id: i32,
    /// The transaction index inside within the aggregate transaction. If the transaction is not an inner transaction, then the secondary id is set to 0.
    #[serde(rename = "secondaryId")]
    pub secondary_id: i32,
}

impl SourceDto {
    /// The transaction that triggered the receipt.
    pub fn new(primary_id: i32, secondary_id: i32) -> SourceDto {
        SourceDto {
            primary_id,
            secondary_id,
        }
    }
}


