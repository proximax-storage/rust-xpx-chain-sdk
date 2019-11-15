#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceDto {
    /// The public key of the owner of the namespace.
    #[serde(rename = "owner")]
    pub owner: String,
    /// The address of the owner of the namespace in hexadecimal.
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "startHeight")]
    pub start_height: Vec<i32>,
    #[serde(rename = "endHeight")]
    pub end_height: Vec<i32>,
    /// The level of the namespace.
    #[serde(rename = "depth")]
    pub depth: i32,
    #[serde(rename = "level0")]
    pub level0: Vec<i32>,
    #[serde(rename = "level1", skip_serializing_if = "Option::is_none")]
    pub level1: Option<Vec<i32>>,
    #[serde(rename = "level2", skip_serializing_if = "Option::is_none")]
    pub level2: Option<Vec<i32>>,
    #[serde(rename = "type")]
    pub _type: crate::models::NamespaceTypeEnum,
    #[serde(rename = "alias")]
    pub alias: crate::models::AliasDto,
    #[serde(rename = "parentId")]
    pub parent_id: Vec<i32>,
}

impl NamespaceDto {
    pub fn new(owner: String, owner_address: String, start_height: Vec<i32>, end_height: Vec<i32>, depth: i32, level0: Vec<i32>, _type: crate::models::NamespaceTypeEnum, alias: crate::models::AliasDto, parent_id: Vec<i32>) -> NamespaceDto {
        NamespaceDto {
            owner,
            owner_address,
            start_height,
            end_height,
            depth,
            level0,
            level1: None,
            level2: None,
            _type,
            alias,
            parent_id,
        }
    }
}


