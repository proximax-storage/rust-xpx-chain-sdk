// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use crate::Handler;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTransactionMetaDto {
    pub channel_name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTransactionInfoDto {
    pub meta: WsTransactionMetaDto,
    transaction: Box<dyn api::TransactionDto>,
}

pub struct HandlerConfirmedAdd {
    pub handler: fn(Box<dyn sdk::transaction::Transaction>)
}

impl Handler for HandlerConfirmedAdd {}
