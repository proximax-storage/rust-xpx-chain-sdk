use std::fmt;

use crate::models::account::{Address, PublicAccount};
use crate::models::mosaic::MosaicId;
use crate::models::namespace::NamespaceId;
use crate::models::transaction::Height;

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

// NamespaceAlias contains aliased mosaicId or address and type of alias
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceAlias {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosaic_id: Option<MosaicId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    pub type_: u8
}

impl fmt::Display for NamespaceAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceInfo {
    pub namespace_id: NamespaceId,
    pub active: bool,
    pub type_space: NamespaceType,
    pub depth: u8,
    pub levels: Vec<NamespaceId>,
    pub alias: NamespaceAlias,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<NamespaceInfo>>,
    pub owner: PublicAccount,
    pub start_height: Height,
    pub end_height: Height,
}

impl NamespaceInfo {
    pub fn get_parent(&self) -> Option<Box<NamespaceInfo>> {
        self.parent.to_owned()
    }
}

impl fmt::Display for NamespaceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceName {
    #[serde(rename = "namespace_id")]
    pub namespace_id: NamespaceId,
    /// The full name of the namespace.
    #[serde(rename = "name")]
    pub name: String,
}

impl fmt::Display for NamespaceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}

#[derive(Serialize, Default, Deserialize)]
pub(crate) struct NamespaceIds {
    /// The array of namespace identifiers.
    #[serde(rename = "namespaceIds")]
    pub namespace_ids: Vec<String>,
}

impl From<Vec<NamespaceId>> for NamespaceIds {
    fn from(e: Vec<NamespaceId>) -> Self {
        let mut ids = NamespaceIds::default();
        for m in e {
            ids.namespace_ids.push(m.to_string())
        }
        return ids;
    }
}