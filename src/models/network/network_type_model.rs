use ::core::fmt;
use ::core::fmt::{Debug, Display};

use crate::models::errors_model::{InternalError, ModelError};

/// MIJIN private network identifier. Decimal value = 96.
pub const MIJIN: NetworkType = NetworkType(0x60);

/// MIJIN_TEST private test network identifier. Decimal value = 144.
pub const MIJIN_TEST: NetworkType = NetworkType(0x90);

/// The PUBLIC test network identifier. Decimal value = 184.
pub const PUBLIC: NetworkType = NetworkType(0xb8);

/// The PUBLIC_TEST test network identifier. Decimal value = 168.
pub const PUBLIC_TEST: NetworkType = NetworkType(0xa8);

/// The PRIVATE test network identifier. Decimal value = 200.
pub const PRIVATE: NetworkType = NetworkType(0xc8);

/// The PRIVATE_TEST test network identifier. Decimal value = 176.
pub const PRIVATE_TEST: NetworkType = NetworkType(0xb0);

pub const ALIAS_ADDRESS: NetworkType = NetworkType(0x91);

pub const NOT_SUPPORTED_NET: NetworkType = NetworkType(0);

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, Copy)]// we derive Default in order to use the clear() method in Drop
pub struct NetworkType(pub(crate) u8);

impl NetworkType {
    pub fn from_string(network_type: &str) -> Result<NetworkType, ModelError> {
        match network_type {
            "MIJIN" => Ok(MIJIN),
            "MIJIN_TEST" => Ok(MIJIN_TEST),
            "PUBLIC" => Ok(PUBLIC),
            "PUBLIC_TEST" => Ok(PUBLIC_TEST),
            "PRIVATE" => Ok(PRIVATE),
            "PRIVATE_TEST" => Ok(PRIVATE_TEST),
            "ALIAS_ADDRESS" => Ok(ALIAS_ADDRESS),
            _ => Err(ModelError(InternalError::NetworkTypeError))
        }
    }
}

impl NetworkType {
    pub fn from_int(network_type: u8) -> Result<NetworkType, ModelError> {
        match network_type {
            0x60 => Ok(MIJIN),
            0x90 => Ok(MIJIN_TEST),
            0xb8 => Ok(PUBLIC),
            0xa8 => Ok(PUBLIC_TEST),
            0xc8 => Ok(PRIVATE),
            0xb0 => Ok(PRIVATE_TEST),
            0x91 => Ok(ALIAS_ADDRESS),
            _ => Err(ModelError(InternalError::NetworkTypeError))
        }
    }
}

impl Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "networkType: {}", self.0)
    }
}
