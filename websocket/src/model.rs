// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

pub(crate) type RouterPath = str;

// const routers path for methods SubscribeService
pub(crate) const PATH_BLOCK: &RouterPath = "block";
pub(crate) const PATH_CONFIRMED_ADDED: &RouterPath = "confirmedAdded";
pub(crate) const PATH_UNCONFIRMED_ADDED: &RouterPath = "unconfirmedAdded";
pub(crate) const PATH_UNCONFIRMED_REMOVED: &RouterPath = "unconfirmedRemoved";
pub(crate) const PATH_PARTIAL_ADDED: &RouterPath = "partialAdded";
pub(crate) const PATH_PARTIAL_REMOVED: &RouterPath = "partialRemoved";
pub(crate) const PATH_COSIGNATURE: &RouterPath = "cosignature";
pub(crate) const PATH_STATUS: &RouterPath = "status";

pub trait WsSubscribeDto {
    type Output;

    fn compact(self) -> Self::Output;

    fn name(&self) -> &str;
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct SubscribeDto {
    pub uid: String,
    pub subscribe: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct UnsubscribeDto {
    pub uid: String,
    pub unsubscribe: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WsConnectionResponse {
    pub uid: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WsMessageInfoDTO {
    meta: WsMessageInfoMetaDto,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMessageInfoMetaDto {
    channel_name: String,
    address: String,
}
