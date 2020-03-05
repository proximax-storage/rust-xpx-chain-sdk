#[derive(Serialize, Deserialize)]
pub struct ServerDto {
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfoDto,
}

impl ServerDto {
    pub fn new(server_info: ServerInfoDto) -> Self {
        ServerDto {
            server_info,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfoDto {
    /// The catapult-rest component version.
    #[serde(rename = "restVersion")]
    pub rest_version: String,
    /// The catapult-sdk component version.
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
}

impl ServerInfoDto {
    pub fn new(rest_version: String, sdk_version: String) -> Self {
        ServerInfoDto {
            rest_version,
            sdk_version,
        }
    }
}

