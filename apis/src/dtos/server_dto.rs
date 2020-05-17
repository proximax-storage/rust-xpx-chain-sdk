#[derive(Serialize, Deserialize)]
pub(crate) struct ServerDto {
    server_info: ServerInfoDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ServerInfoDto {
    /// The catapult-rest component version.
    rest_version: String,
    /// The catapult-sdk component version.
    sdk_version: String,
}