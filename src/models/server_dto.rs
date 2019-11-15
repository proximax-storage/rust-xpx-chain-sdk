#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerDto {
    #[serde(rename = "serverInfo")]
    pub server_info: crate::models::ServerInfoDto,
}

impl ServerDto {
    pub fn new(server_info: crate::models::ServerInfoDto) -> ServerDto {
        ServerDto {
            server_info,
        }
    }
}


