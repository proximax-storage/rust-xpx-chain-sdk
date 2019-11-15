#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerInfoDto {
    /// The catapult-rest component version.
    #[serde(rename = "restVersion")]
    pub rest_version: String,
    /// The catapult-sdk component version.
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
}

impl ServerInfoDto {
    pub fn new(rest_version: String, sdk_version: String) -> ServerInfoDto {
        ServerInfoDto {
            rest_version,
            sdk_version,
        }
    }
}


