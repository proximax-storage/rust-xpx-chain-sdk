#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainScoreDto {
    #[serde(rename = "scoreHigh")]
    pub score_high: Vec<i32>,
    #[serde(rename = "scoreLow")]
    pub score_low: Vec<i32>,
}

impl BlockchainScoreDto {
    pub fn new(score_high: Vec<i32>, score_low: Vec<i32>) -> BlockchainScoreDto {
        BlockchainScoreDto {
            score_high,
            score_low,
        }
    }
}


