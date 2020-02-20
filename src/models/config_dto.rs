#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "networkConfig")]
    pub network_config: String,
    #[serde(rename = "supportedEntityVersions")]
    pub supported_entity_versions: String,
}

impl ConfigDto {
    pub fn new(height: Vec<i32>, network_config: String, supported_entity_versions: String) -> Self {
        ConfigDto {
            height,
            network_config,
            supported_entity_versions,
        }
    }
}


