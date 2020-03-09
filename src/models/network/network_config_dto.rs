use crate::models::config_dto::ConfigDto;
use crate::models::uint_64::Uint64Dto;

#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigDto {
    #[serde(rename = "networkConfig")]
    pub network_config: ConfigDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigBodyDto {
    pub apply_height_delta: Uint64Dto,
    pub network_config: String,
    pub supported_entity_versions: String,
}

/// NetworkConfigTransactionDto : Transaction that updates config.
#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigTransactionDto {
    pub signature: String,
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
    pub apply_height_delta: Uint64Dto,
    pub network_config: String,
    pub supported_entity_versions: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbeddedNetworkConfigTransactionDto {
    pub signer: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: u16,
    pub max_fee: Uint64Dto,
    pub deadline: Uint64Dto,
    pub apply_height_delta: Uint64Dto,
    pub network_config: String,
    pub supported_entity_versions: String,
}