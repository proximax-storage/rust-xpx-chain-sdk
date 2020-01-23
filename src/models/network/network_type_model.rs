#[derive(Debug, PartialEq)]
pub struct NetworkType {
    /// The name of the network.
    #[serde(rename = "name")]
    pub name: String,
    /// A short text describing the network.
    #[serde(rename = "description")]
    pub description: String,
}

impl NetworkType {
    pub fn new(name: String, description: String) -> NetworkTypeDto {
        NetworkTypeDto {
            name,
            description,
        }
    }
}
