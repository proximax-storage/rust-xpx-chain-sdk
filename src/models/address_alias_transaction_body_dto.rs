#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressAliasTransactionBodyDto {
    #[serde(rename = "aliasAction")]
    pub alias_action: crate::models::AliasActionEnum,
    #[serde(rename = "namespaceId")]
    pub namespace_id: Vec<i32>,
    /// The aliased address in hexadecimal.
    #[serde(rename = "address")]
    pub address: String,
}

impl AddressAliasTransactionBodyDto {
    pub fn new(alias_action: crate::models::AliasActionEnum, namespace_id: Vec<i32>, address: String) -> AddressAliasTransactionBodyDto {
        AddressAliasTransactionBodyDto {
            alias_action,
            namespace_id,
            address,
        }
    }
}


