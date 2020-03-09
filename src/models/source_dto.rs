/// SourceDto : The transaction that triggered the receipt.
#[derive(Serialize, Deserialize)]
pub struct SourceDto {
    /// The transaction index within the block.
    pub primary_id: i32,
    /// The transaction index inside within the aggregate transaction. If the transaction is not an inner transaction, then the secondary id is set to 0.
    pub secondary_id: i32,
}


