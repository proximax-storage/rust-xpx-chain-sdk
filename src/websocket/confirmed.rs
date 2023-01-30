/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::api::TransactionDto;

use super::Handler;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WsTransactionMetaDto {
    pub channel_name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WsTransactionInfoDto {
    pub meta: WsTransactionMetaDto,
    transaction: Box<dyn TransactionDto>,
}

pub struct HandlerConfirmedAdd {
    pub handler: Box<dyn Fn(Box<dyn crate::transaction::Transaction>) -> bool + Sync + Send>,
}

impl Handler for HandlerConfirmedAdd {}
