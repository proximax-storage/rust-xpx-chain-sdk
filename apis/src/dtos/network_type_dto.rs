#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct NetworkTypeDto {
    /// The name of the network.
    #[serde(rename = "name")]
    pub name: String,
    /// A short text describing the network.
    #[serde(rename = "description")]
    pub description: String,
}