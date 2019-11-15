/// TransactionStatementDto : The collection of receipts related to a transaction.



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatementDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "source")]
    pub source: crate::models::SourceDto,
    /// The array of receipts.
    #[serde(rename = "receipts")]
    pub receipts: Vec<crate::models::AnyOfBalanceTransferReceiptDtoBalanceChangeReceiptDtoArtifactExpiryReceiptDtoInflationReceiptDto>,
}

impl TransactionStatementDto {
    /// The collection of receipts related to a transaction.
    pub fn new(height: Vec<i32>, source: crate::models::SourceDto, receipts: Vec<crate::models::AnyOfBalanceTransferReceiptDtoBalanceChangeReceiptDtoArtifactExpiryReceiptDtoInflationReceiptDto>) -> TransactionStatementDto {
        TransactionStatementDto {
            height,
            source,
            receipts,
        }
    }
}


