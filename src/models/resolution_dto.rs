use crate::models::source_dto::SourceDto;
use crate::models::uint_64::Uint64Dto;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolutionEntryDto {
    #[serde(rename = "source")]
    pub source: SourceDto,
    #[serde(rename = "resolved")]
    pub resolved: Uint64Dto,
}

impl ResolutionEntryDto {
    pub fn new(source: SourceDto, resolved: Uint64Dto) -> ResolutionEntryDto {
        ResolutionEntryDto {
            source,
            resolved,
        }
    }
}

/// ResolutionStatementDto : A resolution statement keeps the relation between a namespace alias used in a transaction and the real address or mosaic_id.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResolutionStatementDto {
    #[serde(rename = "height")]
    pub height: Uint64Dto,
    #[serde(rename = "unresolved")]
    pub unresolved: Uint64Dto,
    /// The array of resolution entries linked to the unresolved namespace_id. It is an array instead of a single UInt64 field since within one block the resolution might change for different sources due to alias related transactions.
    #[serde(rename = "resolutionEntries")]
    pub resolution_entries: Vec<ResolutionEntryDto>,
}

impl ResolutionStatementDto {
    /// A resolution statement keeps the relation between a namespace alias used in a transaction and the real address or mosaic_id.
    pub fn new(height: Uint64Dto, unresolved: Uint64Dto, resolution_entries: Vec<ResolutionEntryDto>) -> ResolutionStatementDto {
        ResolutionStatementDto {
            height,
            unresolved,
            resolution_entries,
        }
    }
}

