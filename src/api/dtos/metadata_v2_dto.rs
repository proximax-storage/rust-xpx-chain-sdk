/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::account::Address;
use crate::api::Pagination;
use crate::AsUint64;
use crate::helpers::hex_decode;
use crate::metadata::{MetadataEntry, MetadataV2Type, SearchMetadataEntry};

use super::Uint64Dto;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetadataV2InfoDTO {
    metadata_entry: MetadataEntryDto,
    #[serde(flatten)]
    r#abstract: AbstractIdV2DTO,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AbstractIdV2DTO {
    id: Option<String>,
    meta: Option<MetadataDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MetadataDTO {
    id: String,
}

impl MetadataV2InfoDTO {
    pub fn compact(&self) -> anyhow::Result<MetadataEntry> {
        let dto = self.metadata_entry.clone();

        let source_address = Address::from_encoded(dto.source_address)?;
        let scoped_metadata_key = dto.scoped_metadata_key.compact().to_hex();
        let target_id = dto.target_id.compact().to_hex();
        let metadata_type = MetadataV2Type::from(dto.metadata_type);
        let value = String::from_utf8(hex_decode(&dto.value)).unwrap();

        let id = if let Some(ref meta) = self.r#abstract.meta {
            meta.id.to_owned()
        } else {
            self.r#abstract.id.as_ref().unwrap().to_string()
        };

        Ok(MetadataEntry {
            version: dto.version,
            composite_hash: dto.composite_hash,
            source_address,
            target_key: dto.target_key,
            scoped_metadata_key,
            target_id,
            metadata_type,
            value_size: dto.value_size,
            value,
            id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetadataEntryDto {
    version: u8,
    composite_hash: String,
    source_address: String,
    target_key: String,
    scoped_metadata_key: Uint64Dto,
    target_id: Uint64Dto,
    metadata_type: u8,
    value_size: u64,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchMetadataV2InfoDTO {
    data: Vec<MetadataV2InfoDTO>,
    pagination: Pagination,
}

impl SearchMetadataV2InfoDTO {
    pub fn compact(&self) -> anyhow::Result<SearchMetadataEntry> {
        let mut data = vec![];

        for item in &self.data {
            data.push(item.compact()?)
        }
        Ok(SearchMetadataEntry { data, pagination: self.pagination.clone() })
    }
}
