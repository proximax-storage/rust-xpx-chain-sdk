/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

/// total_entries : 6
/// page_number : 1
/// page_size : 20
/// total_pages : 1

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub total_entries: u16,
    pub page_number: u16,
    pub page_size: u16,
    pub total_pages: u16,
}
