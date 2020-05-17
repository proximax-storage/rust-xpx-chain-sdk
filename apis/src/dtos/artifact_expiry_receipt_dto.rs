use super::{ReceiptTypeEnum, Uint64Dto};

/// ArtifactExpiryReceiptDto : An artifact namespace or mosaic expired.
#[derive(Serialize, Deserialize)]
pub(crate) struct ArtifactExpiryReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    version: i32,
    #[serde(rename = "type")]
    _type: ReceiptTypeEnum,
    #[serde(rename = "artifactId")]
    artifact_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ArtifactExpiryReceiptDtoAllOf {
    #[serde(rename = "artifactId")]
    artifact_id: Uint64Dto,
}
