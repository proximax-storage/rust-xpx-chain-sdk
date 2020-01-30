/// An `trait` identifier used to define mosaicId and namespaceId.
pub trait Id {
    /// 64-bit unsigned integer id.
//    fn value(&self) -> Uint64;

    fn to_bytes(&self) -> [u8; 8];

    fn to_hex(&self) -> String;

    fn to_int_array(&self) -> [u32; 2];

//    fn from_u64(value: u64) -> String;
//
//    fn from_hex(value: &str) -> String;
//
//    fn from_ints(value: [u8; 2]) -> String;
}

