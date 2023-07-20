/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use crate::api::OrderV2;
use crate::transaction::{Height, TransactionType};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TransactionSortingField(&'static str);

#[derive(Clone, Debug, Default, Eq, PartialEq, Builder, Serialize)]
#[builder(setter(into, strip_option), default, create_empty = "empty")]
#[serde(rename_all = "camelCase")]
pub struct TransactionQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_number: Option<u16>,
    pub(crate) r#type: Vec<TransactionType>,
    pub(crate) embedded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) order: Option<OrderV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sort_field: Option<TransactionSortingField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to_height: Option<Height>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) from_height: Option<Height>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) height: Option<Height>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) signer_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) recipient_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public_key: Option<String>,
    #[builder(default = "true")]
    pub(crate) first_level: bool, // default true,
}

impl TransactionQueryParams {
    pub fn create() -> TransactionQueryParamsBuilder {
        TransactionQueryParamsBuilder::default()
    }

    /// Serializes a `TransactionQueryParams` into a query string.
    ///
    pub fn to_query_string(&self) -> String {
        qs::to_string(self).unwrap()
    }
}
