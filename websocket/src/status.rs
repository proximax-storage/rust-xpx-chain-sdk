// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use api::Uint64Dto;
use sdk::transaction::{BlockchainTimestamp, Deadline};

use crate::Handler;
use crate::model::WsSubscribeDto;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsStatusMetaDto {
    pub channel_name: String,
    address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsStatusInfoDto {
    pub meta: WsStatusMetaDto,
    #[serde(rename = "status")]
    status: String,
    hash: String,
    deadline: Uint64Dto,
}

impl WsSubscribeDto for WsStatusInfoDto {
    type Output = sdk::transaction::TransactionStatus;

    fn compact(self) -> Self::Output {
        let blockchain_timestamp = BlockchainTimestamp::new(
            self.deadline.compact().to_u64() as i64
        );

        let deadline = Deadline::from(blockchain_timestamp);

        sdk::transaction::TransactionStatus {
            group: "".to_string(),
            status: self.status,
            hash: self.hash,
            deadline: Some(deadline),
            height: None,
        }
    }

    fn name(&self) -> &str {
        &self.meta.channel_name
    }
}

pub struct HandlerStatus {
    pub handler: Box<dyn Fn(sdk::transaction::TransactionStatus) + Send>
}

impl Handler for HandlerStatus {}