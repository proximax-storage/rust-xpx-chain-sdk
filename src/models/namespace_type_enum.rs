/// NamespaceTypeEnum : The namespace type: * 0 -  Root namespace. * 1 -  Subnamespace.

/// The namespace type: * 0 -  Root namespace. * 1 -  Subnamespace. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NamespaceTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}




