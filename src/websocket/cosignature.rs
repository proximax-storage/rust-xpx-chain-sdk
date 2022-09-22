/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use super::Handler;

pub struct HandlerCosignature {
    pub handler: Box<dyn Fn(crate::multisig::CosignatureInfo) -> bool + Sync + Send>,
}

impl Handler for HandlerCosignature {}
