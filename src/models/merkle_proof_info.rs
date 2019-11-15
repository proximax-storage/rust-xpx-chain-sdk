#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MerkleProofInfo {
    /// The complementary data needed to calculate the merkle root.
    #[serde(rename = "merklePath", skip_serializing_if = "Option::is_none")]
    pub merkle_path: Option<Vec<crate::models::MerklePathItem>>,
}

impl MerkleProofInfo {
    pub fn new() -> MerkleProofInfo {
        MerkleProofInfo {
            merkle_path: None,
        }
    }
}


