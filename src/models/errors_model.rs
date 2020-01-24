// rustc seems to think the typenames in match statements (e.g. in
// Display) should be snake cased, for some reason.
#![allow(non_snake_case)]

use core::fmt;
use core::fmt::Display;

/// Internal errors.  Most application-level developers will likely not
/// need to pay any attention to these.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum InternalError {
    NetworkTypeError,

    InvalidAddressError,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InternalError::NetworkTypeError
            => write!(f, "Network type is unknown"),
            InternalError::InvalidAddressError
            => write!(f, "Wrong address"),
        }
    }
}

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
