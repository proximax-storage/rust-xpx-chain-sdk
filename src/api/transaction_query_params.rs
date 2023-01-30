/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

use crate::api::{OrderV2, QueryParam};
use crate::transaction::{Height, TransactionType};

// class TransactionSearchMosaic {
//   TransactionSearchMosaic(this.transferMosaicId);
//
//   final String transferMosaicId;
//   Uint64? toTransferAmount;
//   Uint64? fromTransferAmount;
//
//   Iterable<QueryParam> to_query_params() {
//     final params = <QueryParam>[];
//     toJson().forEach((key, value) => params.add(QueryParam(key, value)));
//     return params;
//   }
//
//   @override
//   String toString() => encoder.convert(this);
//
//   Map<String, dynamic> toJson() {
//     final Map<String, dynamic> val = {'transferMosaicId': transferMosaicId};
//
//     void writeNotNull(String key, value) {
//       if (value != null) {
//         val[key] = value;
//       }
//     }
//
//     writeNotNull('toTransferAmount', toTransferAmount);
//     writeNotNull('fromTransferAmount', fromTransferAmount);
//
//     return val;
//   }
// }

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TransactionSortingField(&'static str);

impl TransactionSortingField {
    pub const BLOCK: TransactionSortingField = TransactionSortingField("meta.height");
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TransactionFieldOrder {
    pub order: OrderV2,
    pub sorting_field: TransactionSortingField,
}

impl TransactionFieldOrder {
    pub fn set_ascending(sorting_field: TransactionSortingField) -> Self {
        Self {
            order: OrderV2::ASC,
            sorting_field,
        }
    }

    pub fn set_descending(sorting_field: TransactionSortingField) -> Self {
        Self {
            order: OrderV2::DESC,
            sorting_field,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<u16>,
    pub r#type: Vec<TransactionType>,
    pub embedded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<TransactionSortingField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_height: Option<Height>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_height: Option<Height>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Height>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    pub first_level: bool, // default true,
}

impl TransactionQueryParams {
    pub fn update_field_order(&mut self, transaction_field_order: TransactionFieldOrder) {
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

impl Default for TransactionQueryParams {
    #[inline]
    fn default() -> Self {
        TransactionQueryParams {
            page_size: None,
            page_number: None,
            r#type: vec![],
            embedded: false,
            order: None,
            sort_field: None,
            to_height: None,
            from_height: None,
            height: None,
            signer_public_key: None,
            recipient_address: None,
            address: None,
            public_key: None,
            first_level: true,
        }
    }
}
