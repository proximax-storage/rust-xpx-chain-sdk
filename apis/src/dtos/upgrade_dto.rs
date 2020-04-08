#[derive(Serialize, Deserialize)]
pub(crate) struct UpgradeDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "blockChainVersion")]
    pub block_chain_version: Vec<i32>,
}