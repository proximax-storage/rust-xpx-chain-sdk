use tower::util::{BoxService, Either};

use crate::api::connection::{Connection, Error, Request, Response};

pub use self::client::Client;
pub use self::node::*;

pub mod body;
mod client;
pub(crate) mod connection;
mod discover;
mod executor;
mod node;
mod reconnect;
pub(crate) mod transport;

pub type Svc = Either<Connection, BoxService<Request, Response, Error>>;
