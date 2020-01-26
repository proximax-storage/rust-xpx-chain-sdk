use byteorder::{BigEndian, ByteOrder};
use core::fmt;

type Err = std::num::ParseIntError;

/// Represents a 64-bit unsigned integer.
///
/// This class uses Dart's native number type `u64` and has a value check for big integers.
/// `u64` will be translated correctly into JavaScript (supported by dart2js).
/// Value range is 0 through 18446744073709551615.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]// we derive Default in order to use the clear() method in Drop
pub struct Uint64(pub(crate) u64);

impl Uint64 {
    pub fn from_ints(lower: u32, higher: u32) -> Uint64 {
        let mut buf = [0; 8];
        BigEndian::write_u32(&mut buf[..4], higher);
        BigEndian::write_u32(&mut buf[4..], lower);
        Uint64(BigEndian::read_u64(&buf))
    }

    pub fn from_bytes(mut b: [u8; 8]) -> Uint64 {
        b.reverse();
        Uint64(BigEndian::read_u64(&b))
    }

    pub fn from_hex(hex_code: &str) -> Result<Uint64, Err> {
            let r: u64 = u64::from_str_radix(&hex_code, 16)?;
            Ok(Uint64(r))
    }

    pub fn to_hex(&self) -> String {
       format!("{:x}", &self.0)
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        BigEndian::write_u64(&mut buf[..8], self.0);
        buf.reverse();
        return buf;
    }

    pub fn to_int_array(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        BigEndian::write_u64(&mut buf[..8], self.0);
        buf.reverse();
        return buf;
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
