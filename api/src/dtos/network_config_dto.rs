// Copyright 2018 ProximaX Limited. All rights reserved.
// Use of this source code is governed by the Apache 2.0
// license that can be found in the LICENSE file.

use super::{ConfigDto, Uint64Dto};
use crate::AbstractTransactionDto;

#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigDto {
    #[serde(rename = "networkConfig")]
    network_config: ConfigDto,
}

/// NetworkConfigTransactionDto : Transaction that updates config.
#[derive(Serialize, Deserialize)]
pub(crate) struct NetworkConfigTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    apply_height_delta: Uint64Dto,
    network_config: String,
    supported_entity_versions: String,
}
