#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkTypeDto {
    /// The name of the network.
    #[serde(rename = "name")]
    pub name: String,
    /// A short text describing the network.
    #[serde(rename = "description")]
    pub description: String,
}

impl NetworkTypeDto {
    pub fn new(name: String, description: String) -> Self {
        NetworkTypeDto {
            name,
            description,
        }
    }
}


