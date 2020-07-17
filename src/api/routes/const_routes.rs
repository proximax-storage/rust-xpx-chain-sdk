// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

/// Account routes.
pub const ACCOUNTS_ROUTE: &str = "/account";
pub const ACCOUNT_ROUTE: &str = "/account/{accountId}";
pub const ACCOUNT_NAMES_ROUTE: &str = "/account/names";
pub const ACCOUNT_PROPERTIES_ROUTE: &str = "/account/{accountId}/properties";
pub const ACCOUNTS_PROPERTIES_ROUTE: &str = "/account/properties";
pub const MULTISIG_ACCOUNT_ROUTE: &str = "/account/{accountId}/multisig";
pub const MULTISIG_ACCOUNT_GRAPH_INFO_ROUTE: &str = "/account/{accountId}/multisig/graph";
pub const TRANSACTIONS_BY_ACCOUNT_ROUTE: &str = "/account/{publicKey}/transactions";
pub const INCOMING_TRANSACTIONS_ROUTE: &str = "/account/{publicKey}/transactions/incoming";
pub const OUTGOING_TRANSACTIONS_ROUTE: &str = "/account/{publicKey}/transactions/outgoing";
pub const UNCONFIRMED_TRANSACTIONS_ROUTE: &str = "/account/{publicKey}/transactions/unconfirmed";
pub const AGGREGATE_TRANSACTIONS_ROUTE: &str = "/account/{publicKey}/transactions/partial";

/// Block routes.
pub const BLOCK_BY_HEIGHT_ROUTE: &str = "/block/{height}";
pub const BLOCK_GET_TRANSACTION_ROUTE: &str = "/block/{height}/transactions";
pub const BLOCK_INFO_ROUTE: &str = "/blocks/{height}/limit/{limit}";

/// Chain routes.
pub const CHAIN_STORAGE_ROUTE: &str = "/diagnostic/storage";
pub const CHAIN_SCORE_ROUTE: &str = "/chain/score";
pub const CHAIN_HEIGHT_ROUTE: &str = "/chain/height";

///  Exchange routes
pub const EXCHANGE_ROUTE: &str = "/account/{account_id}/exchange";
pub const OFFERS_BY_MOSAIC_ROUTE: &str = "/exchange/{offer_type}/{mosaic_id}";

/// Mosaic routes.
pub const MOSAICS_ROUTE: &str = "/mosaic";
pub const MOSAIC_ROUTE: &str = "/mosaic/{mosaic_id}";
pub const MOSAIC_NAMES_ROUTE: &str = "/mosaic/names";

/// Namespace routes.
pub const NAMESPACE_ROUTE: &str = "/namespace/{namespaceId}";
pub const NAMESPACES_FROM_ACCOUNTS_ROUTE: &str = "/account/namespaces";
pub const NAMESPACE_NAMES_ROUTE: &str = "/namespace/names";
pub const NAMESPACES_FROM_ACCOUNT_ROUTES: &str = "/account/{accountId}/namespaces";

/// Node routes.
pub const NODE_INFO: &str = "/node/info";
pub const NODE_TIME: &str = "/node/time";

/// Transaction routes.
pub const TRANSACTIONS_ROUTE: &str = "/transaction";
pub const TRANSACTION_ROUTE: &str = "/transaction/{transactionId}";
pub const TRANSACTION_STATUS_ROUTE: &str = "/transaction/{hash}/status";
pub const TRANSACTIONS_STATUS_ROUTE: &str = "/transaction/statuses";
pub const ANNOUNCE_AGGREGATE_ROUTE: &str = "/transaction/partial";
pub const ANNOUNCE_AGGREGATE_COSIGNATURE_ROUTE: &str = "/transaction/cosignature";
