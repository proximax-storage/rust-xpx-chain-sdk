#[derive(Serialize, Deserialize)]
pub struct ConfigDto {
    height: Vec<i32>,
    network_config: String,
    supported_entity_versions: String,
}


