use super::merkle_model::MerkleProofInfo;

#[derive(Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    #[serde(rename = "payload")]
    pub payload: MerkleProofInfo,
    #[serde(rename = "type")]
    pub _type: String,
}

impl MerkleProofInfoDto {
    pub fn new(payload: MerkleProofInfo, _type: String) -> Self {
        MerkleProofInfoDto {
            payload,
            _type,
        }
    }
}


