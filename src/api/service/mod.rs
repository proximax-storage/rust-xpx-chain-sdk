pub use self::transport::*;

pub(crate) mod transport;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
/// By default, the Err value is of type [`transport::Error`] but this can be overridden if desired.
pub type Result<T, E = Error> = std::result::Result<T, E>;
