use core::{fmt, ptr};
use models::utils::{array_of_u8_to_u32, u32_to_array_of_u8, slice_to_array_of_u8};

/// Represents a 64-bit unsigned integer.
///
/// This class uses Dart's native number type `u64` and has a value check for big integers.
/// `u64` will be translated correctly into JavaScript (supported by dart2js).
/// Value range is 0 through 18446744073709551615.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]// we derive Default in order to use the clear() method in Drop
pub struct Uint64(pub(crate) u64);

impl Uint64 {
    /// Creates a `Uint64` from a pair of u32 integers.
    pub fn from_ints(lower: i32, higher: i32) -> Uint64 {
        let mut buf = [0; 8];

        buf[0] = (lower) as u8;
        buf[1] = (lower >> 8) as u8;
        buf[2] = (lower >> 16) as u8;
        buf[3] = (lower >> 24) as u8;

        buf[4] = (higher) as u8;
        buf[5] = (higher >> 8) as u8;
        buf[6] = (higher >> 16) as u8;
        buf[7] = (higher >> 24) as u8;

        Uint64::from_bytes(buf)
    }

    /// Creates a `Uint64` from a u8 array.
    pub fn from_bytes(mut b: [u8; 8]) -> Uint64 {
        b.reverse();
        Uint64(u64::from_be_bytes(b))
    }

    /// Creates a `Uint64` from a hex &str.
    pub fn from_hex(hex_code: &str) -> Result<Uint64, core::num::ParseIntError> {
        let r: u64 = u64::from_str_radix(&hex_code, 16)?;
        Ok(Uint64(r))
    }

    /// Converts to hex String representation.
    pub fn to_hex(&self) -> String {
        format!("{:x}", &self.0)
    }

    /// Converts to 64-bit byte array.
    pub fn to_bytes(&self) -> [u8; 8] {
        let v = &self.0;
        let mut buf = [0u8; 8];

        buf[0] = *(v) as u8;
        buf[1] = (v >> 8) as u8;
        buf[2] = (v >> 16) as u8;
        buf[3] = (v >> 24) as u8;
        buf[4] = (v >> 32) as u8;
        buf[5] = (v >> 40) as u8;
        buf[6] = (v >> 48) as u8;
        buf[7] = (v >> 56) as u8;

        return buf;
    }

    /// Converts to a pair of i32 integers ([lower, higher]).
    pub fn to_int_array(&self) -> [i32; 2] {
        let bytes = &self.to_bytes();

        let higher: i32 = array_of_u8_to_u32(slice_to_array_of_u8(&bytes[4..]));

        let lower: i32 = array_of_u8_to_u32(slice_to_array_of_u8(&bytes[..4]));

        return [lower, higher];
    }
}

impl fmt::LowerHex for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        write!(f, "{:x}", val) // delegate to u64's implementation
    }
}

impl fmt::UpperHex for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        write!(f, "{:X}", val) // delegate to u64's implementation
    }
}


impl fmt::Binary for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        write!(f, "{:b}", val) // delegate to i32's implementation
    }
}

impl fmt::Display for Uint64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
