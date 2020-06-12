// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use {
    crate::Handler,
    crate::model::WsSubscribeDto,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBlockMetaDto {
    pub channel_name: String,
    hash: String,
    generation_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct WsBlockInfoDto {
    #[serde(rename = "meta")]
    pub meta: WsBlockMetaDto,
    #[serde(rename = "block")]
    pub block: api::BlockDto,
}

impl WsSubscribeDto for WsBlockInfoDto {
    type Output = sdk::blockchain::BlockInfo;

    fn compact(self) -> Self::Output {
        self.block
            .compact(self.meta.generation_hash, 0, [0, 0]).unwrap()
    }

    fn name(&self) -> &str {
        &self.meta.channel_name
    }
}

pub struct HandlerBlock {
    pub handler: Box<dyn Fn(sdk::blockchain::BlockInfo) -> bool + Send>
}

impl Handler for HandlerBlock {}