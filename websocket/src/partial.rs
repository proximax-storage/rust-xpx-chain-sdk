// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use crate::Handler;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsPartialMetaDto {
    pub channel_name: String,
    address: String,
    hash: String,
}

pub struct HandlerPartialAdd {
    pub handler: Box<dyn Fn(sdk::transaction::AggregateTransaction) -> bool + Send>
}

impl Handler for HandlerPartialAdd {}