// Namespace errors
pub static ERR_NAMESPACE_TOO_MANY_PART: &str = "Too many parts";
pub static ERR_NULL_NAMESPACE_ID: &str = "Namespace_id is null or zero";
pub static ERR_WRONG_BIT_NAMESPACE_ID: &str = "Namespace_id doesn't have 64th bit";
pub static ERR_EMPTY_NAMESPACE_IDS: &str = "Namespace ids vec must not by empty";
pub static ERR_EMPTY_NAMESPACE_NAME: &str = "Namespace name must not by empty";
pub static ERR_INVALID_NAMESPACE_NAME: &str = "Namespace name is invalid";

// Mosaic errors
pub static ERR_EMPTY_MOSAIC_IDS: &str = "Mosaics ids vec must not by empty";
pub static ERR_NIL_MOSAIC_ID: &str ="MosaicId must not be null";
pub static ERR_WRONG_BIT_MOSAIC_ID: &str ="MosaicId has 64th bit";
pub static ERR_INVALID_OWNER_PUBLIC_KEY: &str ="Public owner key is invalid";
pub static ERR_NIL_MOSAIC_PROPERTIES: &str ="Mosaic properties must not be null";

// Transaction errors
pub static ERR_EMPTY_TRANSACTION_HASHES: &str = "Transaction hashes vec must not by empty";
pub static ERR_EMPTY_TRANSACTION_IDS: &str = "Transaction ids vec must not by empty";

// Common errors
pub static ERR_NIL_ASSET_ID: &str = "AssetId should not be null";
pub static ERR_EMPTY_ASSET_IDS: &str = "AssetId's vec should not be empty";
pub static ERR_UNKNOWN_BLOCKCHAIN_TYPE: &str = "Not supported Blockchain Type";
pub static ERR_INVALID_HASH_LENGTH: &str = "The length of Hash is invalid";
pub static ERR_INVALID_HASH_HEX: &str = "Invalid Hash it's not hex";
pub static ERR_EMPTY_HASH: &str = "Hash must not by empty";
pub static ERR_INVALID_SIGNATURE_LENGTH: &str = "The length of Signature is invalid";
