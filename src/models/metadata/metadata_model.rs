/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use num_enum::IntoPrimitive;
use serde::{Serialize, Serializer};

/// MetadataTypeEnum :
///The type of the metadata:
///* 1 - Address metadata.
///* 2 - Mosaic metadata.
///* 3 - Namespace metadata.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, IntoPrimitive)]
#[repr(u8)]
pub enum MetadataType {
    MetadataNone,
    MetadataAddressType,
    MetadataMosaicType,
    MetadataNamespaceType,
}

impl MetadataType {
    pub fn value(self) -> u8 {
        self.into()
    }

    pub fn to_bytes(self) -> &'static [u8] {
        &[self.value()]
    }
}

impl From<u8> for MetadataType {
    fn from(num: u8) -> Self {
        match num {
            1 => MetadataType::MetadataAddressType,
            2 => MetadataType::MetadataMosaicType,
            3 => MetadataType::MetadataNamespaceType,
            _ => MetadataType::MetadataNone,
        }
    }
}

impl core::fmt::Display for MetadataType {
    fn fmt(&self, e: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(e, "{:?}", &self)
    }
}

impl Serialize for MetadataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.value())
    }
}
