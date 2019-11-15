/// RolesTypeEnum : The role of the node: * 1 - A peer node. * 2 - An api node.

/// The role of the node: * 1 - A peer node. * 2 - An api node. 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RolesTypeEnum {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,

}




