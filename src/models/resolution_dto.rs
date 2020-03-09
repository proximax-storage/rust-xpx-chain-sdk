use crate::models::source_dto::SourceDto;
use crate::models::uint_64::Uint64Dto;

#[derive(Serialize, Deserialize)]
pub(crate) struct ResolutionEntryDto {
    #[serde(rename = "source")]
    pub source: SourceDto,
    #[serde(rename = "resolved")]
    pub resolved: Uint64Dto,
}

/// ResolutionStatementDto : A resolution statement keeps the relation between a namespace alias used in a transaction and the real address or mosaic_id.
#[derive(Serialize, Deserialize)]
pub(crate) struct ResolutionStatementDto {
    pub height: Uint64Dto,
    pub unresolved: Uint64Dto,
    /// The array of resolution entries linked to the unresolved namespace_id. It is an array instead of a single UInt64 field since within one block the resolution might change for different sources due to alias related transactions.
    pub resolution_entries: Vec<ResolutionEntryDto>,
}

