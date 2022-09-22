// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

pub(super) type RouterPath = str;

// const routers path for methods SubscribeService
pub(super) const PATH_BLOCK: &RouterPath = "block";
pub(super) const PATH_CONFIRMED_ADDED: &RouterPath = "confirmedAdded";
pub(super) const PATH_UNCONFIRMED_ADDED: &RouterPath = "unconfirmedAdded";
pub(super) const PATH_UNCONFIRMED_REMOVED: &RouterPath = "unconfirmedRemoved";
pub(super) const PATH_PARTIAL_ADDED: &RouterPath = "partialAdded";
pub(super) const PATH_PARTIAL_REMOVED: &RouterPath = "partialRemoved";
pub(super) const PATH_COSIGNATURE: &RouterPath = "cosignature";
pub(super) const PATH_STATUS: &RouterPath = "status";

pub(crate) trait WsSubscribeDto {
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
pub(crate) struct WsMessageInfoDTO {
    meta: WsMessageInfoMetaDto,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WsMessageInfoMetaDto {
    channel_name: String,
    address: String,
}
