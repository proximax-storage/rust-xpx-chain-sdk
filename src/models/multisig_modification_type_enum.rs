/// MultisigModificationTypeEnum : The type of the modification: * 0 - Add cosignatory. * 1 - Remove cosignatory.

/// The type of the modification: * 0 - Add cosignatory. * 1 - Remove cosignatory. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MultisigModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}




