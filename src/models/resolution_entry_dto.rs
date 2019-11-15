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


