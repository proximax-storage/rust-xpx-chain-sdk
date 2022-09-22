// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

/// SourceDto : The transaction that triggered the receipt.
#[derive(Serialize, Deserialize)]
pub(crate) struct SourceDto {
    /// The transaction index within the block.
    primary_id: i32,
    /// The transaction index inside within the aggregate transaction. If the transaction is not an inner transaction, then the secondary id is set to 0.
    secondary_id: i32,
}
