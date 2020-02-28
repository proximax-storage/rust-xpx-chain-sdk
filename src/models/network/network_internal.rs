use super::NetworkType;

pub fn extract_network_type(version: i32) -> NetworkType {
    NetworkType::from((version >> 24) as u8)
}
