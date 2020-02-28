/// NamespaceTypeEnum :
/// The namespace type:
/// * 0 -  Root namespace.
/// * 1 -  Subnamespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NamespaceType { Root, Sub }

impl From<u8> for NamespaceType {
    fn from(e: u8) -> Self {
        let mut _type = NamespaceType::Root;
        if e != 0 {
            _type = NamespaceType::Sub;
        }
        _type
    }
}