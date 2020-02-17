use crate::models::resolution_dto::ResolutionStatementDto;
use crate::models::source_dto::SourceDto;

/// StatementsDto : The collection of transaction statements and resolutions triggered for the block requested.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StatementsDto {
    /// The array of transaction statements for the block requested.
    #[serde(rename = "transactionStatements")]
    pub transaction_statements: Vec<TransactionStatementDto>,
    /// The array of address resolutions for the block requested.
    #[serde(rename = "addressResolutionStatements")]
    pub address_resolution_statements: Vec<ResolutionStatementDto>,
    /// The array of mosaic resolutions for the block requested.
    #[serde(rename = "mosaicResolutionStatements")]
    pub mosaic_resolution_statements: Vec<ResolutionStatementDto>,
}

impl StatementsDto {
    /// The collection of transaction statements and resolutions triggered for the block requested.
    pub fn new(transaction_statements: Vec<TransactionStatementDto>, address_resolution_statements: Vec<ResolutionStatementDto>, mosaic_resolution_statements: Vec<ResolutionStatementDto>) -> StatementsDto {
        StatementsDto {
            transaction_statements,
            address_resolution_statements,
            mosaic_resolution_statements,
        }
    }
}

/// TransactionStatementDto : The collection of receipts related to a transaction.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatementDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "source")]
    pub source: SourceDto,
    /// The array of receipts.
    #[serde(rename = "receipts")]
    pub receipts: Vec<String>,
}

impl TransactionStatementDto {
    /// The collection of receipts related to a transaction.
    pub fn new(height: Vec<i32>, source: SourceDto, receipts: Vec<String>) -> TransactionStatementDto {
        TransactionStatementDto {
            height,
            source,
            receipts,
        }
    }
}
