/// ArtifactExpiryReceiptDto : An artifact namespace or mosaic expired.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactExpiryReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::ReceiptTypeEnum,
    #[serde(rename = "artifactId")]
    pub artifact_id: Vec<i32>,
}

impl ArtifactExpiryReceiptDto {
    /// An artifact namespace or mosaic expired.
    pub fn new(version: i32, _type: crate::models::ReceiptTypeEnum, artifact_id: Vec<i32>) -> ArtifactExpiryReceiptDto {
        ArtifactExpiryReceiptDto {
            version,
            _type,
            artifact_id,
        }
    }
}

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


