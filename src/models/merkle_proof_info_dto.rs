use crate::models::merkle_proof_info::MerkleProofInfo;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    #[serde(rename = "payload")]
    pub payload: MerkleProofInfo,
    #[serde(rename = "type")]
    pub _type: String,
}

impl MerkleProofInfoDto {
    pub fn new(payload: MerkleProofInfo, _type: String) -> MerkleProofInfoDto {
        MerkleProofInfoDto {
            payload,
            _type,
        }
    }
}


