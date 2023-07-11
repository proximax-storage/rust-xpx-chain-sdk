use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
pub use self::transport::*;

pub(crate) mod transport;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
/// By default, the Err value is of type [`transport::Error`] but this can be overridden if desired.
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn trace_init() {
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(LevelFilter::DEBUG)
        .try_init()
        .unwrap();
}
