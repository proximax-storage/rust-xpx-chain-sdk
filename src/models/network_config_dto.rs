#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfigDto {
    #[serde(rename = "networkConfig")]
    pub network_config: crate::models::ConfigDto,
}

impl NetworkConfigDto {
    pub fn new(network_config: crate::models::ConfigDto) -> NetworkConfigDto {
        NetworkConfigDto {
            network_config,
        }
    }
}


