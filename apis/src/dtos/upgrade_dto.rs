#[derive(Serialize, Deserialize)]
pub(crate) struct UpgradeDto {
    #[serde(rename = "height")]
    height: Vec<i32>,
    #[serde(rename = "blockChainVersion")]
    block_chain_version: Vec<i32>,
}