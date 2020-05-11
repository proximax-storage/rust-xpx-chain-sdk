use super::{ReceiptTypeEnum, Uint64Dto};

/// ArtifactExpiryReceiptDto : An artifact namespace or mosaic expired.
#[derive(Serialize, Deserialize)]
pub(crate) struct ArtifactExpiryReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: ReceiptTypeEnum,
    #[serde(rename = "artifactId")]
    pub artifact_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ArtifactExpiryReceiptDtoAllOf {
    #[serde(rename = "artifactId")]
    pub artifact_id: Uint64Dto,
}
