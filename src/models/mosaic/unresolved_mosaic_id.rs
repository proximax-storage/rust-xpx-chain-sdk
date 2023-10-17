/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::{any::Any, fmt};

use anyhow::{anyhow, Result};

use crate::AsUint64;
use crate::helpers::{hex_decode, is_hex};
use crate::namespace::NamespaceId;

use super::MosaicId;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AssetIdType {
    #[serde(rename = "NamespaceId")]
    NamespaceIdType,
    #[serde(rename = "MosaicId")]
    MosaicIdType,
}

impl fmt::Display for AssetIdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}

/// An `trait` is used to define mosaicIds and namespaceIds
#[typetag::serde]
pub trait UnresolvedMosaicId: Send + Sync {
    fn to_u64(&self) -> u64;
    fn box_clone(&self) -> Box<dyn UnresolvedMosaicId>;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn get_type(&self) -> AssetIdType;
    /// Create Builder object
    fn to_builder(&self) -> [u8; 8] {
        self.to_u64().to_le_bytes()
    }
    fn to_hex(&self) -> String {
        format!("{:X}", self.to_u64())
    }
    fn to_dto(&self) -> [u32; 2] {
        self.to_u64().to_dto()
    }
}

impl Clone for Box<dyn UnresolvedMosaicId> {
    fn clone(&self) -> Box<dyn UnresolvedMosaicId> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn UnresolvedMosaicId {
    fn eq(&self, other: &Self) -> bool {
        self.to_u64() == other.to_u64()
    }
}

impl<'a> PartialEq for Box<dyn UnresolvedMosaicId> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl fmt::Display for dyn UnresolvedMosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for dyn UnresolvedMosaicId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl dyn UnresolvedMosaicId {
    /// Map asset mosaic hex string to MosaicId or NamespaceId.
    ///
    pub fn from_hex(hex: &str) -> Result<Box<dyn UnresolvedMosaicId>> {
        ensure!(is_hex(hex), "Input string is not in valid hexadecimal notation.");
        let bytes = hex_decode(hex);
        let byte0 = &bytes[0];

        // if most significant bit of byte 0 is set, then we have a namespaceId
        if (byte0 & 128) == 128 {
            Ok(Box::new(NamespaceId::try_from_hex(hex)?))
        } else {
            // most significant bit of byte 0 is not set => mosaicId
            Ok(Box::new(MosaicId::try_from_hex(hex)?))
        }
    }

    pub fn as_mosaic_id(&self) -> Result<MosaicId> {
        match self.get_type() {
            AssetIdType::MosaicIdType => Ok(MosaicId::from(self.to_u64())),
            _ => Err(anyhow!(
				"Wrong MosaicId type; original UnresolvedMosaicId type: {:?}",
				self.get_type()
			)),
        }
    }

    pub fn as_namespace_id(&self) -> Result<NamespaceId> {
        match self.get_type() {
            AssetIdType::NamespaceIdType => Ok(NamespaceId::from(self.to_u64())),
            _ => Err(anyhow!(
				"Wrong NamespaceId type; original UnresolvedMosaicId type: {:?}",
				self.get_type()
			)),
        }
    }

    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast_ref`.
    ///
    pub fn downcast_ref<T: 'static + UnresolvedMosaicId>(&self) -> &T {
        self.try_downcast_ref::<T>()
            .unwrap_or_else(|| panic!("downcast to wrong type; original `UnresolvedAddress` type"))
    }

    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast_ref<T: 'static + UnresolvedMosaicId>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the `UnresolvedAddress` type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast`.
    ///
    pub fn downcast<T: 'static + UnresolvedMosaicId>(self: Box<Self>) -> Box<T> {
        self.try_downcast().unwrap_or_else(|err| panic!("{}", err))
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast<T: 'static + UnresolvedMosaicId>(self: Box<Self>) -> Result<Box<T>> {
        if self.as_ref().as_any().is::<T>() {
            Ok(self.into_any().downcast().unwrap())
        } else {
            Err(anyhow!(
				"downcast to wrong UnresolvedAddress type; original UnresolvedAddress type"
			))
        }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::mosaic::{MosaicId, UnresolvedMosaicId};
    use crate::namespace::NamespaceId;

    lazy_static! {
		pub static ref MOSAIC_ID: MosaicId = MosaicId::try_from_hex("11F4B1B3AC033DB5").unwrap();
		pub static ref NAMESPACE_ID: NamespaceId =
			NamespaceId::try_from_hex("9550CA3FC9B41FC5").unwrap();
	}

    #[test]
    fn test_can_map_hex_string_to_mosaic_id() {
        let unresolved = <dyn UnresolvedMosaicId>::from_hex(MOSAIC_ID.to_hex().as_str()).unwrap();
        assert!(unresolved.clone().try_downcast::<MosaicId>().is_ok());
        assert_eq!(unresolved.try_downcast::<NamespaceId>().is_ok(), false);
    }

    #[test]
    fn test_can_map_hex_string_to_namespace_id() {
        let unresolved =
            <dyn UnresolvedMosaicId>::from_hex(NAMESPACE_ID.to_hex().as_str()).unwrap();
        assert!(unresolved.clone().try_downcast::<NamespaceId>().is_ok());
        assert_eq!(unresolved.try_downcast::<MosaicId>().is_ok(), false);
    }

    #[test]
    #[should_panic(expected = "Input string is not in valid hexadecimal notation.")]
    fn test_should_panic_if_id_not_in_hex() {
        let _ = <dyn UnresolvedMosaicId>::from_hex("test").unwrap();
    }
}
