#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainUpgradeBodyDto {
    #[serde(rename = "upgradePeriod")]
    pub upgrade_period: Vec<i32>,
    #[serde(rename = "newBlockChainVersion")]
    pub new_block_chain_version: Vec<i32>,
}

impl BlockchainUpgradeBodyDto {
    pub fn new(upgrade_period: Vec<i32>, new_block_chain_version: Vec<i32>) -> BlockchainUpgradeBodyDto {
        BlockchainUpgradeBodyDto {
            upgrade_period,
            new_block_chain_version,
        }
    }
}


