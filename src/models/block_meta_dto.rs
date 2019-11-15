#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockMetaDto {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "generationHash")]
    pub generation_hash: String,
    #[serde(rename = "subCacheMerkleRoots")]
    pub sub_cache_merkle_roots: Vec<String>,
    #[serde(rename = "totalFee")]
    pub total_fee: Vec<i32>,
    #[serde(rename = "numTransactions")]
    pub num_transactions: i32,
    #[serde(rename = "numStatements", skip_serializing_if = "Option::is_none")]
    pub num_statements: Option<i32>,
}

impl BlockMetaDto {
    pub fn new(hash: String, generation_hash: String, sub_cache_merkle_roots: Vec<String>, total_fee: Vec<i32>, num_transactions: i32) -> BlockMetaDto {
        BlockMetaDto {
            hash,
            generation_hash,
            sub_cache_merkle_roots,
            total_fee,
            num_transactions,
            num_statements: None,
        }
    }
}


