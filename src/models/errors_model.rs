// rustc seems to think the typenames in match statements (e.g. in
// Display) should be snake cased, for some reason.
#![allow(non_snake_case)]

use core::fmt;

use failure::_core::fmt::Display;

/// Internal errors.  Most application-level developers will likely not
/// need to pay any attention to these.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum InternalError {
    NetworkTypeError,
    /// An error in the length of bytes handed to a constructor.
    ///
    /// To use this, pass a string specifying the `name` of the type which is
    /// returning the error, and the `length` in bytes which its constructor
    /// expects.
    BytesLengthError {
        name: &'static str,
        length: usize,
    },
}

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InternalError::NetworkTypeError
            => write!(f, "Network type is unknown"),
            InternalError::BytesLengthError { name: n, length: l }
            => write!(f, "{} must be {} bytes in length", n, l),
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
