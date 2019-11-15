#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountMetaDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "merkleComponentHash")]
    pub merkle_component_hash: String,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "id")]
    pub id: String,
}

impl AccountMetaDto {
    pub fn new(height: Vec<i32>, hash: String, merkle_component_hash: String, index: i32, id: String) -> AccountMetaDto {
        AccountMetaDto {
            height,
            hash,
            merkle_component_hash,
            index,
            id,
        }
    }
}


