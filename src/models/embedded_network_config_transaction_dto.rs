#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedNetworkConfigTransactionDto {
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
    #[serde(rename = "applyHeightDelta")]
    pub apply_height_delta: Vec<i32>,
    /// Network config like a raw text.
    #[serde(rename = "networkConfig")]
    pub network_config: String,
    /// Allowed versions of transaction in json format.
    #[serde(rename = "supportedEntityVersions")]
    pub supported_entity_versions: String,
}

impl EmbeddedNetworkConfigTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, apply_height_delta: Vec<i32>, network_config: String, supported_entity_versions: String) -> EmbeddedNetworkConfigTransactionDto {
        EmbeddedNetworkConfigTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            apply_height_delta,
            network_config,
            supported_entity_versions,
        }
    }
}


