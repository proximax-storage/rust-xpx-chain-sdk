/*
 * Copyright 2018 ProximaX Limited. All rights reserved.
 * Use of this source code is governed by the Apache 2.0
 * license that can be found in the LICENSE file.
 */

use std::{any::Any, fmt};

use anyhow::{anyhow, Result};

use crate::account::Address;
use crate::helpers::{hex_decode, is_hex};
use crate::namespace::NamespaceId;
use crate::network::NetworkType;

///  Custom trait for unresolved address
///
#[typetag::serde]
pub trait UnresolvedAddress: Sync + Send
    where
        Self: fmt::Debug,
{
    fn recipient_to_string(&self) -> String;
    fn unresolved_address_to_bytes(&self, network_type: NetworkType) -> Vec<u8>;
    fn box_clone(&self) -> Box<dyn UnresolvedAddress>;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl Clone for Box<dyn UnresolvedAddress + 'static> {
    fn clone(&self) -> Box<dyn UnresolvedAddress + 'static> {
        self.box_clone()
    }
}

impl<'a> PartialEq for &'a dyn UnresolvedAddress {
    fn eq(&self, other: &Self) -> bool {
        self.recipient_to_string() == other.recipient_to_string()
    }
}

impl<'a> PartialEq for Box<dyn UnresolvedAddress + 'static> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl fmt::Display for dyn UnresolvedAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.recipient_to_string())
    }
}

impl dyn UnresolvedAddress {
    /// Map asset address string to Address or NamespaceId.
    pub fn from_unresolved_address(hex: &str) -> Result<Box<dyn UnresolvedAddress>> {
        ensure!(is_hex(hex), "Input string is not in valid hexadecimal notation.");

        let bytes = hex_decode(&hex[0..2]);
        let byte0 = &bytes[0];

        // If bit 0 of byte 0 is not set (like in 0x90), then it is a regular address.
        // Else (e.g. 0x91) it represents a namespace id which starts at byte 1.
        if (byte0 & 16) == 16 {
            // namespaceId encoded hexadecimal notation provided
            // only 8 bytes are relevant to resolve the NamespaceId
            let relevant_part = &hex[2..16];
            let mut relevant_part_vec = hex_decode(relevant_part);
            relevant_part_vec.reverse();

            let higher: u32 = u32::from_str_radix(&hex::encode(&relevant_part_vec)[0..8], 16)?;
            let lower: u32 = u32::from_str_radix(&hex::encode(relevant_part_vec)[8..], 16)?;

            Ok(Box::new(NamespaceId::from([higher, lower])))
        } else {
            // most significant bit of byte 0 is not set => mosaicId
            Ok(Box::new(Address::from_encoded(hex)?))
        }
    }

    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast_ref`.
    ///
    pub fn downcast_ref<T: 'static + UnresolvedAddress>(&self) -> &T {
        self.try_downcast_ref::<T>().unwrap_or_else(|| {
            panic!("downcast to wrong UnresolvedAddress type; original `UnresolvedAddress` type")
        })
    }

    /// Downcast a reference to this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast_ref<T: 'static + UnresolvedAddress>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    ///
    /// # Panics
    ///
    /// Panics if the `UnresolvedAddress` type is not `T`. In normal usage, you should know the
    /// specific type. In other cases, use `try_downcast`.
    ///
    pub fn downcast<T: 'static + UnresolvedAddress>(self: Box<Self>) -> Box<T> {
        self.try_downcast().unwrap_or_else(|err| panic!("{}", err))
    }

    /// Downcast this generic `UnresolvedAddress` to a specific type.
    #[inline]
    pub fn try_downcast<T: 'static + UnresolvedAddress>(self: Box<Self>) -> Result<Box<T>> {
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

    use crate::account::{Address, UnresolvedAddress};
    use crate::mosaic::UnresolvedMosaicId;
    use crate::namespace::NamespaceId;

    lazy_static! {
		pub static ref NAMESPACE_ID: NamespaceId =
			NamespaceId::try_from_hex("9550CA3FC9B41FC5").unwrap();
		pub static ref ADDRESS: Address =
			Address::from_raw("VDPI5RSUIFID32NBPLEGHELVDJFXPWQXCRVN3PLR").unwrap();
	}

    #[test]
    fn test_can_map_hex_str_to_address() {
        let unresolved =
            <dyn UnresolvedAddress>::from_unresolved_address(&ADDRESS.encode_as_hex()).unwrap();
        assert_eq!(unresolved.clone().try_downcast::<Address>().is_ok(), true);
        assert_eq!(unresolved.try_downcast::<NamespaceId>().is_ok(), false);
    }

    #[test]
    fn test_can_map_hex_str_to_namespace_id() {
        let unresolved =
            <dyn UnresolvedAddress>::from_unresolved_address(&NAMESPACE_ID.to_hex()).unwrap();
        assert_eq!(unresolved.clone().try_downcast::<NamespaceId>().is_ok(), true);
        assert_eq!(unresolved.try_downcast::<Address>().is_ok(), false);
    }

    #[test]
    #[should_panic(expected = "Input string is not in valid hexadecimal notation.")]
    fn test_should_panic_if_id_not_in_hex() {
        let _ = <dyn UnresolvedAddress>::from_unresolved_address("test").unwrap();
    }
}
