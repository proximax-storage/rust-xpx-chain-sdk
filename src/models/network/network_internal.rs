/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::NetworkType;

pub(crate) fn extract_network_type(version: u32) -> NetworkType {
    NetworkType::from((version >> 24) as u8)
}
