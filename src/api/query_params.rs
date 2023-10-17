/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum Order {
    ASC,
    DESC,
}

impl Order {
    pub fn as_str(self) -> &'static str {
        match self {
            Order::ASC => "id",
            Order::DESC => "-id",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum OrderV2 {
    ASC,
    DESC,
}

impl OrderV2 {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderV2::ASC => "asc",
            OrderV2::DESC => "desc",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryParam {
    pub name: String,
    pub value: Value,
}

/// The `QueryParams` structure describes pagination params for requests.
#[derive(Clone, Debug, Serialize)]
pub struct QueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub page_size: u16,
    /// Order of transactions.
    /// DESC. Newer to older.
    ///  ASC. Older to newer.
    pub order: Order,
}

impl QueryParams {
    pub fn to_query_params(&self) -> Vec<QueryParam> {
        let binding = serde_json::to_value(&self).unwrap();
        let object = binding.as_object().unwrap();

        object
            .into_iter()
            .map(|(key, value)| QueryParam { name: key.to_string(), value: value.clone() })
            .collect::<Vec<_>>()
    }
}
