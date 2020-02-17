use crate::models::Uint64;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    /// The public key used to identify the node.
    pub public_key: String,
    /// The port used for the communication.
    pub port: u16,
    pub network_identifier: u8,
    /// The version of the application.
    pub version: u16,
    pub roles: u8,
    /// The IP address of the endpoint.
    pub host: String,
    /// The name of the node.
    pub friendly_name: String,
}

impl core::fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeTime {
    #[serde(rename = "sendTimestamp", skip_serializing_if = "Option::is_none")]
    pub send_timestamp: Option<Uint64>,
    #[serde(rename = "receiveTimestamp", skip_serializing_if = "Option::is_none")]
    pub receive_timestamp: Option<Uint64>,
}

impl core::fmt::Display for NodeTime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f, "{}",
            serde_json::to_string_pretty(self).unwrap_or_default()
        )
    }
}
