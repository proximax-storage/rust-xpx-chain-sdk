#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedBlockchainUpgradeTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network. 
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
    #[serde(rename = "upgradePeriod")]
    pub upgrade_period: Vec<i32>,
    #[serde(rename = "newBlockChainVersion")]
    pub new_block_chain_version: Vec<i32>,
}

impl EmbeddedBlockchainUpgradeTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, upgrade_period: Vec<i32>, new_block_chain_version: Vec<i32>) -> EmbeddedBlockchainUpgradeTransactionDto {
        EmbeddedBlockchainUpgradeTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            upgrade_period,
            new_block_chain_version,
        }
    }
}


