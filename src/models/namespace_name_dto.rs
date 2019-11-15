#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceNameDto {
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

impl NamespaceNameDto {
    pub fn new(namespace_id: Vec<i32>, name: String) -> NamespaceNameDto {
        NamespaceNameDto {
            namespace_id,
            name,
        }
    }
}


