use crate::models::*;

/// The `NamespaceId` id structure describes namespace id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamespaceId {
    #[serde(rename = "id")]
    pub id: Uint64,
    /// The full name can be empty when the namespace id is created using only the `Uint64` id.
    #[serde(rename = "fullName")]
    pub full_name: String,
}

//impl NamespaceId {
//    /// Creates a new `NamespaceId` from a `Uint64`.
//    pub fn from_uin64(uin64: Uint64) -> NamespaceId {
//        NamespaceId(uin64)
//    }
//
//    /// Creates a new `NamespaceId` from a hex string.
//    pub fn from_hex(string_hex: &str) -> Result<NamespaceId, ModelError> {
//        if string_hex.is_empty() {
//            return Err(ModelError(InternalError::HexEmptyError));
//        }
//
//        if !::models::account::is_hex(string_hex) {
//            return Err(ModelError(InternalError::InvalidHex));
//        };
//
//        Ok(NamespaceId(Uint64::from_hex(string_hex).unwrap()))
//    }
//
//    /// Creates a new `NamespaceId` from a pair of 32-bit integers.
//    pub fn from_ints(lower: u32, higher: u32) -> NamespaceId {
//        NamespaceId(Uint64::from_ints(lower, higher))
//    }
//
//    /// Creates a new `NamespaceId` from a given `MosaicNonce` and owner's `PublicAccount`.
//    pub fn from_nonce_and_owner(nonce: MosaicNonce, owner_public_id: PublicAccount) -> NamespaceId {
//        let id = generate_mosaic_id(nonce, owner_public_id);
//        NamespaceId(id)
//    }
//}
//
//impl Id for NamespaceId {
//    fn to_bytes(&self) -> [u8; 8] {
//        let id = &self.0;
//        id.to_bytes()
//    }
//
//    fn to_hex(&self) -> String {
//        let id = &self.0;
//        id.to_hex()
//    }
//
//    fn to_int_array(&self) -> [u32; 2] {
//        let id = &self.0;
//        id.to_int_array()
//    }
//
//    fn eq(&self, other: &Id) -> bool {
//        &self.to_bytes() == &other.to_bytes()
//    }
//}
//
//impl fmt::Display for NamespaceId {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:X}", self.0)
//    }
//}
