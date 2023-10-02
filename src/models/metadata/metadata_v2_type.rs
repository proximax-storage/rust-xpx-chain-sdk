/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::fmt;

use {
    num_enum::IntoPrimitive,
    serde::{Serialize, Serializer},
};

///The type of the metadata:
///* 1 - Account metadata.
///* 2 - Mosaic metadata.
///* 3 - Namespace metadata.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MetadataV2Type {
    MetadataAccount,
    MetadataMosaic,
    MetadataNamespace,
    MetadataNone,
}

impl MetadataV2Type {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn to_bytes(&self) -> [u8; 1] {
        [self.value()]
    }
}

impl From<u8> for MetadataV2Type {
    fn from(num: u8) -> Self {
        use MetadataV2Type::*;
        match num {
            0 => MetadataAccount,
            1 => MetadataMosaic,
            2 => MetadataNamespace,
            _ => MetadataNone,
        }
    }
}

impl fmt::Display for MetadataV2Type {
    fn fmt(&self, e: &mut fmt::Formatter) -> fmt::Result {
        write!(e, "{:?}", &self)
    }
}

impl Serialize for MetadataV2Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(self.value())
    }
}
