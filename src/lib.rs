extern crate chrono;
#[macro_use]
extern crate downcast_rs;
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

pub use self::apis::sirius_client;
pub use self::models::account;
pub use self::models::message;
pub use self::models::mosaic;
pub use self::models::namespace;
pub use self::models::network;
pub use self::models::transaction;
pub use self::models::Uint64;

mod apis;
mod models;

type Result<T> = ::std::result::Result<T, failure::Error>;

