/// LinkActionEnum : The type of the action: * 0 - Link. * 1 - Unlink.
/// The type of the action: * 0 - Link. * 1 - Unlink.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LinkActionEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}

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

/// AliasActionEnum : The alias action: * 0 -  Link alias. * 1 -  Unlink alias.
/// The alias action: * 0 -  Link alias. * 1 -  Unlink alias.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AliasActionEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
}
