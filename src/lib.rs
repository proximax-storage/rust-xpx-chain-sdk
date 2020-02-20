extern crate chrono;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate failure;
extern crate flatbuffers as fb;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;
extern crate url;
extern crate xpx_crypto;

pub mod apis;
pub mod models;

type Result<T> = ::std::result::Result<T, failure::Error>;

