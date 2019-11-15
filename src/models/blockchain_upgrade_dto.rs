#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainUpgradeDto {
    #[serde(rename = "blockchainUpgrade")]
    pub blockchain_upgrade: crate::models::UpgradeDto,
}

impl BlockchainUpgradeDto {
    pub fn new(blockchain_upgrade: crate::models::UpgradeDto) -> BlockchainUpgradeDto {
        BlockchainUpgradeDto {
            blockchain_upgrade,
        }
    }
}


