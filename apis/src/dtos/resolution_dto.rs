use super::{SourceDto, Uint64Dto};

#[derive(Serialize, Deserialize)]
pub(crate) struct ResolutionEntryDto {
    #[serde(rename = "source")]
    source: SourceDto,
    #[serde(rename = "resolved")]
    resolved: Uint64Dto,
}

/// ResolutionStatementDto : A resolution statement keeps the relation between a namespace alias used in a transaction and the real address or mosaic_id.
#[derive(Serialize, Deserialize)]
pub(crate) struct ResolutionStatementDto {
    height: Uint64Dto,
    unresolved: Uint64Dto,
    /// The array of resolution entries linked to the unresolved namespace_id. It is an array instead of a single UInt64 field since within one block the resolution might change for different sources due to alias related transactions.
    resolution_entries: Vec<ResolutionEntryDto>,
}
