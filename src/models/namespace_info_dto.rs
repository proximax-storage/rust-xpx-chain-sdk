#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceInfoDto {
    #[serde(rename = "meta")]
    pub meta: crate::models::NamespaceMetaDto,
    #[serde(rename = "namespace")]
    pub namespace: crate::models::NamespaceDto,
}

impl NamespaceInfoDto {
    pub fn new(meta: crate::models::NamespaceMetaDto, namespace: crate::models::NamespaceDto) -> NamespaceInfoDto {
        NamespaceInfoDto {
            meta,
            namespace,
        }
    }
}


