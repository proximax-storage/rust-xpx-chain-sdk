/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

/// The query params structure describes pagination by page number and page size.
///
#[derive(Clone, Debug, Eq, PartialEq, Builder, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageQueryParams {
    // Page number should be non negative number, otherwise 0 (first page).
    pub page: u16,

    // Page size between 10 and 100, otherwise 25.
    pub page_size: u8,
}

impl PageQueryParams {
    pub fn create(page: u16, page_size: u8) -> PageQueryParams {
        let page_size = if page_size >= 10 && page_size <= 100 { page_size } else { 25 };

        PageQueryParams { page, page_size }
    }

    /// Serializes a `PageQueryParams` into a query string.
    ///
    pub fn to_query_string(&self) -> String {
        qs::to_string(self).unwrap()
    }
}

impl Default for PageQueryParams {
    fn default() -> Self {
        PageQueryParams { page: 0, page_size: 25 }
    }
}
