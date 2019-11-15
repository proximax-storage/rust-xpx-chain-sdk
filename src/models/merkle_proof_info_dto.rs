#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    #[serde(rename = "payload")]
    pub payload: crate::models::MerkleProofInfo,
    #[serde(rename = "type")]
    pub _type: String,
}

impl MerkleProofInfoDto {
    pub fn new(payload: crate::models::MerkleProofInfo, _type: String) -> MerkleProofInfoDto {
        MerkleProofInfoDto {
            payload,
            _type,
        }
    }
}


