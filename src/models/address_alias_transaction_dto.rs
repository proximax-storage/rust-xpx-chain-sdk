/// AddressAliasTransactionDto : Transaction that attaches a namespace to an account.



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressAliasTransactionDto {
    /// The signature of the entity. The signature was generated by the signer and can be used to validate tha the entity data was not modified by a node. 
    #[serde(rename = "signature")]
    pub signature: String,
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
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
}

impl AddressAliasTransactionDto {
    /// Transaction that attaches a namespace to an account.
    pub fn new(signature: String, signer: String, version: i32, _type: crate::models::EntityTypeEnum, max_fee: Vec<i32>, deadline: Vec<i32>, alias_action: crate::models::AliasActionEnum, namespace_id: Vec<i32>, address: String) -> AddressAliasTransactionDto {
        AddressAliasTransactionDto {
            signature,
            signer,
            version,
            _type,
            max_fee,
            deadline,
            alias_action,
            namespace_id,
            address,
        }
    }
}


