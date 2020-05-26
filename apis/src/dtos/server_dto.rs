// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

#[derive(Serialize, Deserialize)]
pub(crate) struct ServerDto {
    server_info: ServerInfoDto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ServerInfoDto {
    /// The catapult-rest component version.
    rest_version: String,
    /// The catapult-sdk component version.
    sdk_version: String,
}