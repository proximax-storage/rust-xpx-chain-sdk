use super::NetworkType;

pub(crate) fn extract_network_type(version: u32) -> NetworkType {
    NetworkType::from((version >> 24) as u8)
}
