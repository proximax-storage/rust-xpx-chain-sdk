/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{ResolutionStatementDto, SourceDto};

/// StatementsDto : The collection of transaction statements and resolutions triggered for the block requested.
#[derive(Serialize, Deserialize)]
pub(crate) struct StatementsDto {
    /// The array of transaction statements for the block requested.
    transaction_statements: Vec<TransactionStatementDto>,
    /// The array of address resolutions for the block requested.
    address_resolution_statements: Vec<ResolutionStatementDto>,
    /// The array of mosaic resolutions for the block requested.
    mosaic_resolution_statements: Vec<ResolutionStatementDto>,
}

/// TransactionStatementDto : The collection of receipts related to a transaction.
#[derive(Serialize, Deserialize)]
pub(crate) struct TransactionStatementDto {
    height: Vec<i32>,
    source: SourceDto,
    /// The array of receipts.
    receipts: Vec<String>,
}
