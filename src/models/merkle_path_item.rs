#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MerklePathItem {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

impl MerklePathItem {
    pub fn new() -> MerklePathItem {
        MerklePathItem {
            position: None,
            hash: None,
        }
    }
}


