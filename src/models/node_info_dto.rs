#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoDto {
    /// The public key used to identify the node.
    #[serde(rename = "publicKey")]
    pub public_key: String,
    /// The port used for the communication.
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "networkIdentifier")]
    pub network_identifier: i32,
    /// The version of the application.
    #[serde(rename = "version")]
    pub version: i32,
    #[serde(rename = "roles")]
    pub roles: crate::models::RolesTypeEnum,
    /// The IP address of the endpoint.
    #[serde(rename = "host")]
    pub host: String,
    /// The name of the node.
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
}

impl NodeInfoDto {
    pub fn new(public_key: String, port: i32, network_identifier: i32, version: i32, roles: crate::models::RolesTypeEnum, host: String, friendly_name: String) -> NodeInfoDto {
        NodeInfoDto {
            public_key,
            port,
            network_identifier,
            version,
            roles,
            host,
            friendly_name,
        }
    }
}


