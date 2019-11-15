/// AliasActionEnum : The alias action: * 0 -  Link alias. * 1 -  Unlink alias.

/// The alias action: * 0 -  Link alias. * 1 -  Unlink alias. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AliasActionEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}




