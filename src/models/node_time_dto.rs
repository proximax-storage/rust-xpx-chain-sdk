#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTimeDto {
    #[serde(rename = "communicationTimestamps")]
    pub communication_timestamps: crate::models::CommunicationTimestamps,
}

impl NodeTimeDto {
    pub fn new(communication_timestamps: crate::models::CommunicationTimestamps) -> NodeTimeDto {
        NodeTimeDto {
            communication_timestamps,
        }
    }
}


