use super::{ResolutionStatementDto, SourceDto};

/// StatementsDto : The collection of transaction statements and resolutions triggered for the block requested.
#[derive(Serialize, Deserialize)]
pub(crate) struct StatementsDto {
    /// The array of transaction statements for the block requested.
    transaction_statements: Vec<TransactionStatementDto>,
    /// The array of address resolutions for the block requested.
    address_resolution_statements: Vec<ResolutionStatementDto>,
    /// The array of mosaic resolutions for the block requested.
    mosaic_resolution_statements: Vec<ResolutionStatementDto>,
}

/// TransactionStatementDto : The collection of receipts related to a transaction.
#[derive(Serialize, Deserialize)]
pub(crate) struct TransactionStatementDto {
    height: Vec<i32>,
    source: SourceDto,
    /// The array of receipts.
    receipts: Vec<String>,
}
