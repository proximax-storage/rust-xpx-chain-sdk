#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfigBodyDto {
    #[serde(rename = "applyHeightDelta")]
    pub apply_height_delta: Vec<i32>,
    /// Network config like a raw text.
    #[serde(rename = "networkConfig")]
    pub network_config: String,
    /// Allowed versions of transaction in json format.
    #[serde(rename = "supportedEntityVersions")]
    pub supported_entity_versions: String,
}

impl NetworkConfigBodyDto {
    pub fn new(apply_height_delta: Vec<i32>, network_config: String, supported_entity_versions: String) -> NetworkConfigBodyDto {
        NetworkConfigBodyDto {
            apply_height_delta,
            network_config,
            supported_entity_versions,
        }
    }
}


