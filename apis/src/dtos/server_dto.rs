#[derive(Serialize, Deserialize)]
pub(crate) struct ServerDto {
    pub server_info: ServerInfoDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ServerInfoDto {
    /// The catapult-rest component version.
    pub rest_version: String,
    /// The catapult-sdk component version.
    pub sdk_version: String,
}