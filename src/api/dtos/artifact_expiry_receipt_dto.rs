/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{ReceiptType, Uint64Dto};

/// ArtifactExpiryReceiptDto : An artifact namespace or mosaic expired.
#[derive(Serialize, Deserialize)]
pub(crate) struct ArtifactExpiryReceiptDto {
    /// The version of the receipt.
    #[serde(rename = "version")]
    version: i32,
    #[serde(rename = "type")]
    _type: ReceiptType,
    #[serde(rename = "artifactId")]
    artifact_id: Uint64Dto,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ArtifactExpiryReceiptDtoAllOf {
    #[serde(rename = "artifactId")]
    artifact_id: Uint64Dto,
}
