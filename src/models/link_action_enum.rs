/// LinkActionEnum : The type of the action: * 0 - Link. * 1 - Unlink.

/// The type of the action: * 0 - Link. * 1 - Unlink. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LinkActionEnum {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,

}




