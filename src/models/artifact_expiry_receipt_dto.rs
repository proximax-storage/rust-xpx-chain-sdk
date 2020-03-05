use crate::models::receipt_dto::ReceiptTypeEnum;
use crate::models::uint_64::Uint64Dto;

/// ArtifactExpiryReceiptDto : An artifact namespace or mosaic expired.
#[derive(Serialize, Deserialize)]
pub struct ArtifactExpiryReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: ReceiptTypeEnum,
    #[serde(rename = "artifactId")]
    pub artifact_id: Uint64Dto,
}

impl ArtifactExpiryReceiptDto {
    /// An artifact namespace or mosaic expired.
    pub fn new(version: i32, _type: ReceiptTypeEnum, artifact_id: Uint64Dto) -> ArtifactExpiryReceiptDto {
        ArtifactExpiryReceiptDto {
            version,
            _type,
            artifact_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArtifactExpiryReceiptDtoAllOf {
    #[serde(rename = "artifactId")]
    pub artifact_id: Uint64Dto,
}

impl ArtifactExpiryReceiptDtoAllOf {
    pub fn new(artifact_id: Uint64Dto) -> ArtifactExpiryReceiptDtoAllOf {
        ArtifactExpiryReceiptDtoAllOf {
            artifact_id,
        }
    }
}


