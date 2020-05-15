/// SourceDto : The transaction that triggered the receipt.
#[derive(Serialize, Deserialize)]
pub struct SourceDto {
    /// The transaction index within the block.
    primary_id: i32,
    /// The transaction index inside within the aggregate transaction. If the transaction is not an inner transaction, then the secondary id is set to 0.
    secondary_id: i32,
}


