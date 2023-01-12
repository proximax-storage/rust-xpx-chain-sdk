pub(crate) use self::add_origin::AddOrigin;
pub(crate) use self::connection::Connection;
pub(crate) use self::connector::connector;
pub(crate) use self::discover::DynamicServiceStream;
pub(crate) use self::executor::SharedExec;
pub(crate) use self::user_agent::UserAgent;

mod add_origin;
mod connection;
mod connector;
mod discover;
pub(crate) mod executor;
// pub(crate) mod http_timeout;
mod io;
mod reconnect;

mod user_agent;

// pub use self::router::Routes;
