#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterNamespaceTransactionBodyDto {
    #[serde(rename = "namespaceType")]
    pub namespace_type: crate::models::NamespaceTypeEnum,
    #[serde(rename = "duration")]
    pub duration: Vec<i32>,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The unique namespace name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId")]
    pub parent_id: Vec<i32>,
}

impl RegisterNamespaceTransactionBodyDto {
    pub fn new(namespace_type: crate::models::NamespaceTypeEnum, duration: Vec<i32>, namespace_id: Vec<i32>, name: String, parent_id: Vec<i32>) -> RegisterNamespaceTransactionBodyDto {
        RegisterNamespaceTransactionBodyDto {
            namespace_type,
            duration,
            namespace_id,
            name,
            parent_id,
        }
    }
}


