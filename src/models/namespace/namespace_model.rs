/// NamespaceTypeEnum :
/// The namespace type:
/// * 0 -  Root namespace.
/// * 1 -  Subnamespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NamespaceType { Root, Sub }

impl From<u8> for NamespaceType {
    fn from(num: u8) -> Self {
        match num {
            1 => NamespaceType::Sub,
            _ => NamespaceType::Root
        }
    }
}