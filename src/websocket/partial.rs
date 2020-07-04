/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{Handler, WsUnconfirmedMetaDto};

pub struct HandlerPartialAdd {
    pub handler: Box<dyn Fn(crate::transaction::AggregateTransaction) -> bool + Send>,
}

impl Handler for HandlerPartialAdd {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsPartialRemoveDto {
    meta: WsUnconfirmedMetaDto,
}

impl WsPartialRemoveDto {
    pub fn compact(&self) -> crate::transaction::TransactionInfo {
        self.meta.compact()
    }
}

pub struct HandlerPartialRemove {
    pub handler: Box<dyn Fn(crate::transaction::TransactionInfo) -> bool + Send>,
}

impl Handler for HandlerPartialRemove {}
