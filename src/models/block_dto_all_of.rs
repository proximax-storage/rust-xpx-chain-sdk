#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockDtoAllOf {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "timestamp")]
    pub timestamp: Vec<i32>,
    #[serde(rename = "difficulty")]
    pub difficulty: Vec<i32>,
    /// The fee multiplier applied to transactions contained in block.
    #[serde(rename = "feeMultiplier")]
    pub fee_multiplier: i32,
    /// The hash of the previous block.
    #[serde(rename = "previousBlockHash")]
    pub previous_block_hash: String,
    /// The transactions included in a block are hashed forming a merkle tree. The root of the tree summarizes them. 
    #[serde(rename = "blockTransactionsHash")]
    pub block_transactions_hash: String,
    /// The collection of receipts  are hashed into a merkle tree and linked  to a block. The block header stores the root hash. 
    #[serde(rename = "blockReceiptsHash")]
    pub block_receipts_hash: String,
    /// For each block, the state of the blockchain is stored in RocksDB,  forming a patricia tree. The root of the tree summarizes the state of the blockchain for the given block. 
    #[serde(rename = "stateHash")]
    pub state_hash: String,
    /// The public key of the optional beneficiary designated by harvester.
    #[serde(rename = "beneficiary")]
    pub beneficiary: String,
    /// The part of the transaction fee harvester is willing to get. From 0 up to FeeInterestDenominator. The customer gets (FeeInterest / FeeInterestDenominator)'th part of the maximum transaction fee.
    #[serde(rename = "feeInterest")]
    pub fee_interest: i32,
    /// Denominator of the transaction fee.
    #[serde(rename = "feeInterestDenominator")]
    pub fee_interest_denominator: i32,
}

impl BlockDtoAllOf {
    pub fn new(height: Vec<i32>, timestamp: Vec<i32>, difficulty: Vec<i32>, fee_multiplier: i32, previous_block_hash: String, block_transactions_hash: String, block_receipts_hash: String, state_hash: String, beneficiary: String, fee_interest: i32, fee_interest_denominator: i32) -> BlockDtoAllOf {
        BlockDtoAllOf {
            height,
            timestamp,
            difficulty,
            fee_multiplier,
            previous_block_hash,
            block_transactions_hash,
            block_receipts_hash,
            state_hash,
            beneficiary,
            fee_interest,
            fee_interest_denominator,
        }
    }
}


