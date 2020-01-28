// rustc seems to think the typenames in match statements (e.g. in
// Display) should be snake cased, for some reason.
#![allow(non_snake_case)]

use core::fmt;
use core::fmt::Display;

impl ::failure::Fail for InternalError {}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct ModelError(pub(crate) InternalError);

impl Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::failure::Fail for ModelError {
    fn cause(&self) -> Option<&dyn (::failure::Fail)> {
        Some(&self.0)
    }
}

/// Internal errors.  Most application-level developers will likely not
/// need to pay any attention to these.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum InternalError {
    NetworkTypeError,
    InvalidAddressError,
    InvalidSignatureLenError,
    InvalidSignatureHexError,
    VerifyError,
    HexEmptyError,
    InvalidHex,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InternalError::NetworkTypeError
            => write!(f, "Network type is unknown"),
            InternalError::InvalidAddressError
            => write!(f, "Wrong address"),
            InternalError::InvalidSignatureLenError
            => write!(f, "Signature length is incorrect"),
            InternalError::InvalidSignatureHexError
            => write!(f, "Signature must be hexadecimal"),
            InternalError::VerifyError
            => write!(f, "Verification equation was not satisfied"),
            InternalError::HexEmptyError
            => write!(f, "The hex string must not be empty"),
            InternalError::InvalidHex
            => write!(f, "Invalid Hex string"),
        }
    }
}

impl<'a> From<&'a xpx_crypto::SignatureError> for ModelError {
    /// Derive this public key from its corresponding `ExpandedSecretKey`.
    fn from(expanded_model_error: &xpx_crypto::SignatureError) -> ModelError {
        ModelError(InternalError::VerifyError)
    }
}
