use ::core::mem::size_of;

/// The number of bytes in an `byte`.
pub const SIZEOF_BYTE: usize = size_of::<u8>();

/// The number of bytes in an `short`.
pub const SIZEOF_SHORT: usize = size_of::<u16>();

/// The number of bytes in an `int`.
pub const SIZEOF_INT: usize = size_of::<u32>();

///// The number of bytes in an `float`.
//pub const SIZEOF_FLOAT: usize = size_of::<f32>();

///// The number of bytes in an `long`.
//pub const SIZEOF_LONG: usize = size_of::<u64>();

///// The number of bytes in an `double`.
//pub const SIZEOF_DOUBLE: usize = size_of::<f64>();

///// The number of bytes in an file identifier.
//pub const FILE_IDENTIFIER_LENGTH: usize = SIZEOF_INT;
