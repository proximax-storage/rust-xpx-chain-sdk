#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactExpiryReceiptDtoAllOf {
    #[serde(rename = "artifactId")]
    pub artifact_id: Vec<i32>,
}

impl ArtifactExpiryReceiptDtoAllOf {
    pub fn new(artifact_id: Vec<i32>) -> ArtifactExpiryReceiptDtoAllOf {
        ArtifactExpiryReceiptDtoAllOf {
            artifact_id,
        }
    }
}


