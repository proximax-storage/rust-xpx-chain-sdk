/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::network::NetworkType;
use crate::node::{NodeInfo, NodeTime};

use super::Uint64Dto;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfoDto {
    /// The public key used to identify the node.
    pub public_key: String,
    /// The port used for the communication.
    pub port: u16,
    pub network_identifier: u8,
    /// The version of the application.
    pub version: u16,
    pub roles: u8,
    /// The IP address of the endpoint.
    pub host: String,
    /// The name of the node.
    pub friendly_name: String,
}

impl NodeInfoDto {
    pub(crate) fn compact(&self) -> NodeInfo {
        let network_type = NetworkType::try_from(self.network_identifier).unwrap_or_default();

        NodeInfo {
            public_key: self.public_key.to_owned(),
            version: self.version,
            roles: self.roles,
            host: self.host.to_owned(),
            friendly_name: self.friendly_name.to_owned(),
            network_type,
            port: self.port,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NodeTimeDto {
    #[serde(rename = "communicationTimestamps")]
    communication_timestamps: CommunicationTimestampsDto,
}

#[derive(Serialize, Deserialize)]
struct CommunicationTimestampsDto {
    #[serde(rename = "sendTimestamp", skip_serializing_if = "Option::is_none")]
    send_timestamp: Option<Uint64Dto>,
    #[serde(rename = "receiveTimestamp", skip_serializing_if = "Option::is_none")]
    receive_timestamp: Option<Uint64Dto>,
}

impl NodeTimeDto {
    pub(crate) fn compact(&self) -> NodeTime {
        let mut send = u64::default();
        if let Some(value) = &self.communication_timestamps.send_timestamp {
            send = value.compact();
        };

        let mut receive = u64::default();
        if let Some(value) = &self.communication_timestamps.receive_timestamp {
            receive = value.compact();
        };

        NodeTime { send_timestamp: Some(send), receive_timestamp: Some(receive) }
    }
}
