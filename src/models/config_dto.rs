#[derive(Serialize, Deserialize)]
pub struct ConfigDto {
    pub height: Vec<i32>,
    pub network_config: String,
    pub supported_entity_versions: String,
}


