/// StatementsDto : The collection of transaction statements and resolutions triggered for the block requested.



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StatementsDto {
    /// The array of transaction statements for the block requested.
    #[serde(rename = "transactionStatements")]
    pub transaction_statements: Vec<crate::models::TransactionStatementDto>,
    /// The array of address resolutions for the block requested.
    #[serde(rename = "addressResolutionStatements")]
    pub address_resolution_statements: Vec<crate::models::ResolutionStatementDto>,
    /// The array of mosaic resolutions for the block requested.
    #[serde(rename = "mosaicResolutionStatements")]
    pub mosaic_resolution_statements: Vec<crate::models::ResolutionStatementDto>,
}

impl StatementsDto {
    /// The collection of transaction statements and resolutions triggered for the block requested.
    pub fn new(transaction_statements: Vec<crate::models::TransactionStatementDto>, address_resolution_statements: Vec<crate::models::ResolutionStatementDto>, mosaic_resolution_statements: Vec<crate::models::ResolutionStatementDto>) -> StatementsDto {
        StatementsDto {
            transaction_statements,
            address_resolution_statements,
            mosaic_resolution_statements,
        }
    }
}


