//! HTTP specific body utilities.

use http_body::Body;
use super::connection::Error;

/// A type erased HTTP body used for tonic services.
pub type BoxBody = http_body::combinators::UnsyncBoxBody<bytes::Bytes, Error>;

/// Convert a [`http_body::Body`] into a [`BoxBody`].
pub fn boxed_body<B>(body: B) -> BoxBody
    where
        B: Body<Data = bytes::Bytes> + Send + 'static,
        B::Error: Into<Error>,
{
    body.map_err(Into::into).boxed_unsync()
}

// this also exists in `crate::codegen` but we need it here since `codegen` has
// `#[cfg(feature = "codegen")]`.
/// Create an empty `BoxBody`
pub fn empty_body() -> BoxBody {
    http_body::Empty::new()
        .map_err(|err| match err {})
        .boxed_unsync()
}