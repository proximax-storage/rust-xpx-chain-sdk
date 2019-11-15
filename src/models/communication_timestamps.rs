#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationTimestamps {
    #[serde(rename = "sendTimestamp", skip_serializing_if = "Option::is_none")]
    pub send_timestamp: Option<Vec<i32>>,
    #[serde(rename = "receiveTimestamp", skip_serializing_if = "Option::is_none")]
    pub receive_timestamp: Option<Vec<i32>>,
}

impl CommunicationTimestamps {
    pub fn new() -> CommunicationTimestamps {
        CommunicationTimestamps {
            send_timestamp: None,
            receive_timestamp: None,
        }
    }
}


