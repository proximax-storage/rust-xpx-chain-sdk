#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceMetaDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "index")]
    pub index: i32,
}

impl NamespaceMetaDto {
    pub fn new(id: String, active: bool, index: i32) -> NamespaceMetaDto {
        NamespaceMetaDto {
            id,
            active,
            index,
        }
    }
}


