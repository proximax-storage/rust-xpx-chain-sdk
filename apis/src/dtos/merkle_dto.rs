#[derive(Serialize, Deserialize)]
pub struct MerkleProofInfoDto {
    // #[serde(rename = "payload")]
    // pub payload: MerkleProofInfo,
    #[serde(rename = "type")]
    _type: String,
}
