#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceIds {
    /// The array of namespace identifiers.
    #[serde(rename = "namespaceIds", skip_serializing_if = "Option::is_none")]
    pub namespace_ids: Option<Vec<String>>,
}

impl NamespaceIds {
    pub fn new() -> NamespaceIds {
        NamespaceIds {
            namespace_ids: None,
        }
    }
}


