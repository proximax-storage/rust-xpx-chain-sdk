#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageInfoDto {
    /// The number of blocks stored.
    #[serde(rename = "numBlocks")]
    pub num_blocks: i32,
    /// The number of transactions stored.
    #[serde(rename = "numTransactions")]
    pub num_transactions: i32,
    /// The number of accounts created.
    #[serde(rename = "numAccounts")]
    pub num_accounts: i32,
}

impl StorageInfoDto {
    pub fn new(num_blocks: i32, num_transactions: i32, num_accounts: i32) -> StorageInfoDto {
        StorageInfoDto {
            num_blocks,
            num_transactions,
            num_accounts,
        }
    }
}


