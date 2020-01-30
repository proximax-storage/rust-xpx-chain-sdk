use core::fmt;

/// An `trait` identifier used to define mosaic_id and namespaceId.
pub trait Id {
    fn to_bytes(&self) -> [u8; 8];

    fn to_hex(&self) -> String;

    fn to_int_array(&self) -> [u32; 2];

    fn eq(&self, other: &Id) -> bool;
}

impl<'a> PartialEq for &'a Id {
    fn eq(&self, other: &Self) -> bool {
        &self.to_bytes() == &other.to_bytes()
    }
}
