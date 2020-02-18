use crate::models::transaction::EntityVersion;

pub fn extract_version(version: i32) -> EntityVersion {
    return version & 0xFFFFFF;
}
