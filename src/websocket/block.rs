/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::api::BlockDto;
use crate::websocket::{model::WsSubscribeDto, Handler};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WsBlockMetaDto {
    pub channel_name: String,
    hash: String,
    generation_hash: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WsBlockInfoDto {
    #[serde(rename = "meta")]
    pub meta: WsBlockMetaDto,
    #[serde(rename = "block")]
    pub block: BlockDto,
}

impl WsSubscribeDto for WsBlockInfoDto {
    type Output = crate::blockchain::BlockInfo;

    fn compact(self) -> Self::Output {
        self.block
            .compact(self.meta.generation_hash, 0, [0, 0])
            .unwrap()
    }

    fn name(&self) -> &str {
        &self.meta.channel_name
    }
}

pub struct HandlerBlock {
    pub handler: Box<dyn Fn(crate::blockchain::BlockInfo) -> bool + Sync + Send>,
}

impl Handler for HandlerBlock {}
