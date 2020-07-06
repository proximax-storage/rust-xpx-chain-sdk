/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::{AbstractTransactionDto, FieldDto, MetadataModificationDto};

#[derive(Serialize, Deserialize)]
struct AddressMetadataDto {
    #[serde(rename = "metadataType")]
    metadata_type: u32,
    #[serde(rename = "fields")]
    fields: Vec<FieldDto>,
    #[serde(rename = "metadataId")]
    metadata_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AddressMetadataDtoAllOf {
    #[serde(rename = "metadataId")]
    metadata_id: String,
}

#[derive(Serialize, Deserialize)]
struct AddressMetadataInfoDto {
    #[serde(rename = "metadata")]
    metadata: AddressMetadataDto,
}

/// AddressMetadataTransactionDto : Transaction that addes metadata to account.
#[derive(Serialize, Deserialize)]
struct AddressMetadataTransactionDto {
    #[serde(flatten)]
    r#abstract: AbstractTransactionDto,
    /// The address in hexadecimal.
    metadata_id: String,
    metadata_type: u8,
    /// The array of metadata modifications.
    modifications: Vec<MetadataModificationDto>,
}
