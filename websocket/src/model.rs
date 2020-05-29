// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

pub(crate) type RouterPath = str;

// const routers path for methods SubscribeService
pub(crate) const PATH_BLOCK: &RouterPath = "block";
pub(crate) const PATH_CONFIRMED_ADDED: &RouterPath = "confirmedAdded/{address}";
pub(crate) const PATH_UNCONFIRMED_ADDED: &RouterPath = "unconfirmedAdded/{address}";
pub(crate) const PATH_UNCONFIRMED_REMOVED: &RouterPath = "unconfirmedRemoved/{address}";
pub(crate) const PATH_PARTIAL_ADDED: &RouterPath = "partialAdded";
pub(crate) const PATH_PARTIAL_REMOVED: &RouterPath = "partialRemoved";
pub(crate) const PATH_COSIGNATURE: &RouterPath = "cosignature";
pub(crate) const PATH_STATUS: &RouterPath = "status/{address}";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SubscribeDto {
    pub uid: String,
    pub subscribe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UnsubscribeDto {
    pub uid: String,
    pub unsubscribe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsConnectionResponse {
    pub uid: String
}