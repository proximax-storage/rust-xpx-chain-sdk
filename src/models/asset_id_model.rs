/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use ::core::fmt;

use super::{mosaic::MosaicId, namespace::NamespaceId, Uint64};

/// An `trait` identifier used to define mosaic_id and namespace_id.
pub trait AssetId: Send + Sync + erased_serde::Serialize
where
    Self: fmt::Debug,
{
    fn to_uint64(&self) -> Uint64;

    fn to_u64(&self) -> u64 {
        *self.to_uint64()
    }

    fn to_mosaic_id(&self) -> MosaicId {
        MosaicId::from(self.to_uint64())
    }

    fn to_nemspace_id(&self) -> NamespaceId {
        NamespaceId::from(self.to_uint64())
    }

    fn to_bytes(&self) -> [u8; 8] {
        self.to_uint64().to_bytes()
    }

    fn to_hex(&self) -> String {
        self.to_uint64().to_hex()
    }

    fn to_u32_array(&self) -> [u32; 2] {
        self.to_uint64().to_int_array()
    }

    fn is_empty(&self) -> bool {
        self.to_uint64().to_bytes().len() == 0
    }

    fn box_clone(&self) -> Box<dyn AssetId>;
}

// implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn AssetId + 'static> {
    fn clone(&self) -> Box<dyn AssetId + 'static> {
        self.box_clone()
    }
}

serialize_trait_object!(AssetId);

impl<'a> PartialEq for &'a dyn AssetId {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl fmt::Display for dyn AssetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}
