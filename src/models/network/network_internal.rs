use super::NetworkType;

pub fn extract_network_type(version: i32) -> NetworkType {
    NetworkType((version >> 24) as u8)
}
