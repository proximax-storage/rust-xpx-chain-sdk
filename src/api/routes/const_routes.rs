// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

/// Account routes.
pub const ACCOUNTS_ROUTE: &str = "/account";
pub const ACCOUNT_ROUTE: &str = "/account/{address_id}";
pub const ACCOUNT_NAMES_ROUTE: &str = "/account/names";
pub const ACCOUNT_PROPERTIES_ROUTE: &str = "/account/{address_id}/properties";
pub const ACCOUNTS_PROPERTIES_ROUTE: &str = "/account/properties";
pub const MULTISIG_ACCOUNT_ROUTE: &str = "/account/{address_id}/multisig";
pub const MULTISIG_ACCOUNT_GRAPH_INFO_ROUTE: &str = "/account/{address_id}/multisig/graph";

/// Chain routes.
pub const CHAIN_STORAGE_ROUTE: &str = "/diagnostic/storage";
pub const CHAIN_SCORE_ROUTE: &str = "/chain/score";
pub const CHAIN_HEIGHT_ROUTE: &str = "/chain/height";
pub const BLOCK_BY_HEIGHT_ROUTE: &str = "/block/{height}";
pub const BLOCK_GET_TRANSACTION_ROUTE: &str = "/block/{height}/transactions";
pub const BLOCK_INFO_ROUTE: &str = "/blocks/{height}/limit/{limit}";

///  Exchange routes
pub const EXCHANGE_ROUTE: &str = "/account/{account_id}/exchange";
pub const OFFERS_BY_MOSAIC_ROUTE: &str = "/exchange/{offer_type}/{mosaic_id}";

/// Mosaic routes.
pub const MOSAICS_ROUTE: &str = "/mosaic";
pub const MOSAIC_ROUTE: &str = "/mosaic/{mosaic_id}";
pub const MOSAIC_NAMES_ROUTE: &str = "/mosaic/names";
pub const MOSAIC_RICH_LIST_ROUTE: &str = "/mosaic/{mosaicId}/richlist";
pub const MOSAIC_LEVY_ROUTE: &str = "/mosaic/{mosaicId}/levy";

/// Namespace routes.
pub const NAMESPACE_ROUTE: &str = "/namespace/{namespaceId}";
pub const NAMESPACES_FROM_ACCOUNTS_ROUTE: &str = "/account/namespaces";
pub const NAMESPACE_NAMES_ROUTE: &str = "/namespace/names";
pub const NAMESPACES_FROM_ACCOUNT_ROUTES: &str = "/account/{accountId}/namespaces";

/// Node routes.
pub const NODE_INFO: &str = "/node/info";
pub const NODE_TIME: &str = "/node/time";
pub const NODE_PEERS: &str = "/node/peers";

/// Network routes.
pub const NETWORK_INFO: &str = "/network";

/// Transaction routes.
pub const TRANSACTIONS_ROUTE: &str = "/transactions/confirmed";
pub const TRANSACTIONS_UNCONFIRMED_ROUTE: &str = "/transactions/Unconfirmed";

pub const TRANSACTIONS_BY_GROUP_ROUTE: &str = "/transactions/{group}";
pub const TRANSACTION_ROUTE: &str = "/transactions/{group}/{transactionId}";

pub const TRANSACTION_STATUS_ROUTE: &str = "/transactionStatus/{hash}";
pub const TRANSACTIONS_STATUS_ROUTE: &str = "/transactionStatus";
pub const ANNOUNCE_TRANSACTION_ROUTE: &str = "/transactions";
pub const AGGREGATE_TRANSACTIONS_ROUTE: &str = "/transactions/partial";
pub const ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE: &str = "/transaction/cosignature";

// routes for MetadataService
pub const METADATA_INFO_ROUTE: &str = "/metadata";
pub const METADATA_BY_ACCOUNT_ROUTE: &str = "/account/{address_id}/metadata";
pub const METADATA_BY_MOSAIC_ROUTE: &str = "/mosaic/{mosaic_id}/metadata";
pub const METADATA_BY_NAMESPACE_ROUTE: &str = "/namespace/{namespace_id}/metadata";

pub const METADATA_V2_INFO_ROUTE: &str = "/metadata_v2/{compositeHash}";
pub const METADATAS_V2_INFO_ROUTE: &str = "/metadata_v2";
