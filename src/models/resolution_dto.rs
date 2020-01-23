#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolutionEntryDto {
    #[serde(rename = "source")]
    pub source: crate::models::SourceDto,
    #[serde(rename = "resolved")]
    pub resolved: Vec<i32>,
}

impl ResolutionEntryDto {
    pub fn new(source: crate::models::SourceDto, resolved: Vec<i32>) -> ResolutionEntryDto {
        ResolutionEntryDto {
            source,
            resolved,
        }
    }
}

/// ResolutionStatementDto : A resolution statement keeps the relation between a namespace alias used in a transaction and the real address or mosaicId.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolutionStatementDto {
    #[serde(rename = "height")]
    pub height: Vec<i32>,
    #[serde(rename = "unresolved")]
    pub unresolved: Vec<i32>,
    /// The array of resolution entries linked to the unresolved namespaceId. It is an array instead of a single UInt64 field since within one block the resolution might change for different sources due to alias related transactions.
    #[serde(rename = "resolutionEntries")]
    pub resolution_entries: Vec<crate::models::ResolutionEntryDto>,
}

impl ResolutionStatementDto {
    /// A resolution statement keeps the relation between a namespace alias used in a transaction and the real address or mosaicId.
    pub fn new(height: Vec<i32>, unresolved: Vec<i32>, resolution_entries: Vec<crate::models::ResolutionEntryDto>) -> ResolutionStatementDto {
        ResolutionStatementDto {
            height,
            unresolved,
            resolution_entries,
        }
    }
}

