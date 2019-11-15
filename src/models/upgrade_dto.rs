#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpgradeDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "blockChainVersion")]
    pub block_chain_version: Vec<i32>,
}

impl UpgradeDto {
    pub fn new(height: Vec<i32>, block_chain_version: Vec<i32>) -> UpgradeDto {
        UpgradeDto {
            height,
            block_chain_version,
        }
    }
}


