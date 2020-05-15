use super::{ConfigDto, Uint64Dto};

#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigDto {
    #[serde(rename = "networkConfig")]
    network_config: ConfigDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigBodyDto {
    apply_height_delta: Uint64Dto,
    network_config: String,
    supported_entity_versions: String,
}

/// NetworkConfigTransactionDto : Transaction that updates config.
#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigTransactionDto {
    signature: String,
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    apply_height_delta: Uint64Dto,
    network_config: String,
    supported_entity_versions: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbeddedNetworkConfigTransactionDto {
    signer: String,
    version: u32,
    #[serde(rename = "type")]
    _type: u16,
    max_fee: Uint64Dto,
    deadline: Uint64Dto,
    apply_height_delta: Uint64Dto,
    network_config: String,
    supported_entity_versions: String,
}
