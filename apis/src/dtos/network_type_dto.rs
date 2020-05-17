#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct NetworkTypeDto {
    /// The name of the network.
    #[serde(rename = "name")]
    name: String,
    /// A short text describing the network.
    #[serde(rename = "description")]
    description: String,
}