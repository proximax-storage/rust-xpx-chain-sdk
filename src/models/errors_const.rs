// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

// Common errors
pub const ERR_INVALID_NAMESPACE_ALIASED: &str = "Namespace is not aliased to Mosaic";
pub const ERR_INVALID_ASSET_ID: &str = "AssetId's vector should not be empty";

// Accounts errors
pub const ERR_EMPTY_ADDRESSES_IDS: &str = "list of addresses should not be empty";
pub const ERR_EMPTY_ADDRESSES_ID: &str = "account_id is empty";
pub const ERR_INVALID_ACCOUNT_ID: &str = "Invalid account_id";
pub const ERR_EMPTY_ADDRESSES: &str = "address string must not be empty.";
pub const ERR_INVALID_ADDRESSES_LEN: &str = "Invalid len address.";
pub const ERR_INVALID_ADDRESSES_HEX: &str = "Invalid Address it's not hex.";

// Namespace errors
pub const ERR_NAMESPACE_TOO_MANY_PART: &str = "Too many parts";
pub const ERR_EMPTY_NAMESPACE_ID: &str = "Namespace_id must not be empty";
pub const ERR_WRONG_BIT_NAMESPACE_ID: &str = "Namespace_id doesn't have 64th bit";
pub const ERR_EMPTY_NAMESPACE_IDS: &str = "Namespace ids vec must not by empty";
pub const ERR_EMPTY_NAMESPACE_NAME: &str = "Namespace name must not by empty";
pub const ERR_INVALID_NAMESPACE_NAME: &str = "Namespace name is invalid";

// Mosaic errors
pub const ERR_EMPTY_MOSAIC_IDS: &str = "Mosaics ids vec must not by empty";
pub const ERR_EMPTY_MOSAIC_ID: &str = "MosaicId must not be empty";
pub const ERR_INVALID_MOSAIC_PROPERTIES: &str = "Mosaic Properties is not valid.";
pub const ERR_INVALID_MOSAIC_PROPERTY_ID: &str = "Unknown Property Id.";

// Transaction errors
pub const ERR_EMPTY_TRANSACTION_HASHES: &str = "Transaction hashes vec must not by empty";
pub const ERR_EMPTY_TRANSACTION_IDS: &str = "Transaction ids vec must not by empty";
pub const ERR_EMPTY_INNER_TRANSACTION: &str = "innerTransactions must not be empty";
pub const ERR_EMPTY_MODIFICATIONS: &str = "modifications must not empty";
pub const ERR_EMPTY_TRANSACTION_SIGNER: &str = "some of the transaction does not have a signer";
pub const ERR_EMPTY_COSIGNATURE_HASH: &str = "Cosignature transaction hash it should not be empty";
pub const ERR_EMPTY_GENERATION_HASH: &str = "Generation hash it should not be empty";
pub const ERR_INVALID_AGGREGATE_TRANSACTION: &str =
    "The transaction is not an AggregateTransaction.";

// Common errors
pub const ERR_UNKNOWN_BLOCKCHAIN_TYPE: &str = "Not supported Blockchain Type.";
pub const ERR_INVALID_HASH_LENGTH: &str = "The length of Hash is invalid.";
pub const ERR_INVALID_HASH_HEX: &str = "Invalid Hash it's not hex.";

//pub const ERR_EMPTY_HASH: &str = "Hash must not by empty.";
pub const ERR_EMPTY_NETWORK_TYPE: &str = "NetworkType string is empty.";
pub const ERR_INVALID_PRIVATE_KEY_LENGTH: &str = "The private key string is empty.";
pub const ERR_INVALID_PUBLIC_KEY_LENGTH: &str = "The public key string is empty.";
pub const ERR_INVALID_KEY_LENGTH: &str = "The length of key is invalid.";
pub const ERR_INVALID_KEY_HEX: &str = "Invalid hex key string.";
pub const ERR_INVALID_SIGNATURE_LENGTH: &str = "The length of Signature is invalid.";
pub const ERR_INVALID_SIGNATURE_HEX: &str = "Signature must be hexadecimal.";
pub const ERR_UNKNOWN_TYPE: &str = "Not supported value Type.";
pub const ERR_INVALID_DATA_LENGTH: &str = "The length of data is invalid.";

// Metadata errors
pub const ERR_METADATA_EMPTY_ADDRESSES: &str = "list adresses ids must not by empty.";
pub const ERR_METADATA_EMPTY_MOSAIC_IDS: &str = "list mosaics ids must not by empty.";
pub const ERR_METADATA_EMPTY_NAMESPACE_IDS: &str = "list namespaces ids must not by empty.";
pub const ERR_METADATA_EMPTY_MODIFICATIONS: &str = "modifications must not empty.";
