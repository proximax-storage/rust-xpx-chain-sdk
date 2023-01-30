/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use serde_json::json;

use crate::api::QueryParam;

/// The query params structure describes pagination by page number and page size.
///
pub struct PageQueryParams {
    // Page number should be non negative number, otherwise 0 (first page).
    pub page: u16,

    // Page size between 10 and 100, otherwise 25.
    pub page_size: u8,
}

impl PageQueryParams {
    pub fn create(page: u16, page_size: u8) -> PageQueryParams {
        let page_size = if page_size >= 10 && page_size <= 100 {
            page_size
        } else {
            25
        };

        PageQueryParams { page, page_size }
    }

    pub fn to_query_params(&self) -> Vec<QueryParam> {
        vec![
            QueryParam {
                name: "pageSize".to_string(),
                value: json!(self.page_size),
            },
            QueryParam {
                name: "page".to_string(),
                value: json!(self.page),
            },
        ]
    }
}

impl Default for PageQueryParams {
    fn default() -> Self {
        PageQueryParams {
            page: 0,
            page_size: 25,
        }
    }
}
