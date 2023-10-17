/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::std::str::FromStr;

use crate::{
    api::Uint64Dto,
    transaction::{BlockchainTimestamp, Deadline, TransactionStatus},
};

use super::{Handler, model::WsSubscribeDto};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WsStatusMetaDto {
    pub channel_name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WsStatusInfoDto {
    pub meta: WsStatusMetaDto,
    #[serde(rename = "status")]
    status: String,
    hash: String,
    deadline: Uint64Dto,
}

impl WsSubscribeDto for WsStatusInfoDto {
    type Output = TransactionStatus;

    fn compact(self) -> Self::Output {
        let blockchain_timestamp = BlockchainTimestamp::new(*self.deadline.compact() as i64);

        let deadline = Deadline::from(blockchain_timestamp);

        TransactionStatus {
            group: "".to_string(),
            status: self.status,
            hash: TransactionHash::from_str(&self.hash).unwrap(),
            deadline: Some(deadline),
            height: None,
        }
    }

    fn name(&self) -> &str {
        &self.meta.channel_name
    }
}

pub struct HandlerStatus {
    pub handler: Box<dyn Fn(crate::transaction::TransactionStatus) -> bool + Sync + Send>,
}

impl Handler for HandlerStatus {}
