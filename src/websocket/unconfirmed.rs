/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::Handler;

pub struct HandlerUnconfirmedAdd {
    pub handler: Box<dyn Fn(Box<dyn crate::transaction::Transaction>) -> bool + Send>,
}

impl Handler for HandlerUnconfirmedAdd {}

pub struct HandlerUnconfirmedRemoved {
    pub handler: Box<dyn Fn(crate::transaction::TransactionInfo) -> bool + Send>,
}

impl Handler for HandlerUnconfirmedRemoved {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUnconfirmedRemovedDto {
    meta: WsUnconfirmedMetaDto,
}

impl WsUnconfirmedRemovedDto {
    pub fn compact(&self) -> crate::transaction::TransactionInfo {
        self.meta.compact()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUnconfirmedMetaDto {
    hash: String,
    pub channel_name: String,
    address: String,
}

impl WsUnconfirmedMetaDto {
    pub fn compact(&self) -> crate::transaction::TransactionInfo {
        crate::transaction::TransactionInfo {
            height: crate::Uint64::default(),
            index: 0,
            id: String::new(),
            hash: Some(self.hash.to_owned()),
            merkle_component_hash: None,
            agregate_hash: None,
            aggregate_id: None,
            unique_aggregate_hash: None,
        }
    }
}
