/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

use crate::api::{OrderV2, QueryParam};
use crate::metadata::MetadataV2Type;
use crate::mosaic::UnresolvedMosaicId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MetadataSortingField(&'static str);

impl MetadataSortingField {
    pub const VALUE: MetadataSortingField = MetadataSortingField("metadataEntry.value");
    pub const VALUE_SIZE: MetadataSortingField = MetadataSortingField("metadataEntry.VALUE_SIZE");
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MetadataFieldOrder {
    pub order: OrderV2,
    pub sorting_field: MetadataSortingField,
}

impl MetadataFieldOrder {
    pub fn set_ascending(sorting_field: MetadataSortingField) -> Self {
        Self {
            order: OrderV2::ASC,
            sorting_field,
        }
    }

    pub fn set_descending(sorting_field: MetadataSortingField) -> Self {
        Self {
            order: OrderV2::DESC,
            sorting_field,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<MetadataV2Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<MetadataSortingField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Box<dyn UnresolvedMosaicId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoped_metadata_key: Option<u64>,
}

impl MetadataQueryParams {
    pub fn update_field_order(&mut self, transaction_field_order: MetadataFieldOrder) {
        self.sort_field = Some(transaction_field_order.sorting_field);
    }

    pub fn to_query_params(&self) -> Vec<QueryParam> {
        let binding = serde_json::to_value(&self).unwrap();
        let object = binding.as_object().unwrap();

        let mut query_param = vec![];

        for (key, value) in object {
            if let Value::Array(mut value_vec) = value.clone() {
                value_vec.dedup();
                for value in value_vec {
                    query_param.push(QueryParam {
                        name: format!(r"{key}[]"),
                        value: value.clone(),
                    })
                }
            } else {
                query_param.push(QueryParam {
                    name: key.into(),
                    value: value.clone(),
                })
            }
        }
        query_param
    }
}

impl Default for MetadataQueryParams {
    #[inline]
    fn default() -> Self {
        MetadataQueryParams {
            page_size: None,
            page_number: None,
            metadata_type: None,
            order: None,
            sort_field: None,
            source_address: None,
            target_id: None,
            target_key: None,
            scoped_metadata_key: None,
        }
    }
}
