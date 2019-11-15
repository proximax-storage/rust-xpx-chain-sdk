#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbeddedRegisterNamespaceTransactionDto {
    /// The public key of the entity signer formatted as hexadecimal.
    #[serde(rename = "signer")]
    pub signer: String,
    /// The entity version. The higher byte represents the network identifier: * 0x68 (MAIN_NET) - Public main network. * 0x98 (TEST_NET) - Public test network. * 0x60 (MIJIN) - Private network. * 0x90 (MIJIN_TEST) - Private test network. 
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "type")]
    pub _type: crate::models::EntityTypeEnum,
    #[serde(rename = "max_fee")]
    pub max_fee: Vec<i32>,
    #[serde(rename = "deadline")]
    pub deadline: Vec<i32>,
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

impl EmbeddedRegisterNamespaceTransactionDto {
    pub fn new(signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, namespace_type: crate::models::NamespaceTypeEnum, duration: Vec<i32>, namespace_id: Vec<i32>, name: String, parent_id: Vec<i32>) -> EmbeddedRegisterNamespaceTransactionDto {
        EmbeddedRegisterNamespaceTransactionDto {
            signer,
            version,
            _type,
            max_fee,
            deadline,
            namespace_type,
            duration,
            namespace_id,
            name,
            parent_id,
        }
    }
}


