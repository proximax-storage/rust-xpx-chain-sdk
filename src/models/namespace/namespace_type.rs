/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

/// The namespace type:
/// * 0 -  Root namespace.
/// * 1 -  Subnamespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NamespaceType {
    Root,
    Sub,
}

impl From<u8> for NamespaceType {
    fn from(num: u8) -> Self {
        match num {
            1 => NamespaceType::Sub,
            _ => NamespaceType::Root,
        }
    }
}
