/// AliasTypeEnum : The alias type: * 0 -  No alias. * 1 -  Mosaic id alias. * 2 -  Addres alias.

/// The alias type: * 0 -  No alias. * 1 -  Mosaic id alias. * 2 -  Addres alias. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AliasTypeEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,

}




