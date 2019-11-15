/// AccountPropertiesModificationTypeEnum : The account properties modification type: * 0 - Add property. * 1 - Remove property.

/// The account properties modification type: * 0 - Add property. * 1 - Remove property. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountPropertiesModificationTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}
