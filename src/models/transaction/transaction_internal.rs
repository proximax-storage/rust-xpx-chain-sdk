pub type EntityVersion = u32;

pub fn extract_version(version: i32) -> EntityVersion {
return (version as u32)& 0xFFFFFF
}
