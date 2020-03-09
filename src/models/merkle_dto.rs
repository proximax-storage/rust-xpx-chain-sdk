use super::merkle_model::MerkleProofInfo;

#[derive(Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    #[serde(rename = "payload")]
    pub payload: MerkleProofInfo,
    #[serde(rename = "type")]
    pub _type: String,
}