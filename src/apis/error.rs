use hyper::http::uri::InvalidUri;
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiriusError {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    SiriusError(SiriusError),
    UriError(InvalidUri),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Failure(failure::Error),
}

impl ::failure::Fail for Error {}

#[derive(Debug)]
pub struct ApiError {
    pub code: hyper::StatusCode,
    pub content: Option<String>,
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e);
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e);
    }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Self {
        return Error::Failure(e);
    }
}

impl From<SiriusError> for Error {
    fn from(e: SiriusError) -> Self {
        return Error::SiriusError(e);
    }
}

impl From<InvalidUri> for Error {
    fn from(e: InvalidUri) -> Self {
        return Error::UriError(e);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.to_owned() {
            Error::SiriusError(e) => write!(f, "{{ code: \"{}\", message: \"{}\" }}", e.code, e.message),
            Error::UriError(e) => write!(f, "{}", e),
            Error::Hyper(e) => write!(f, "{}", e),
            Error::Serde(e) => write!(f, "{}", e),
            Error::Failure(e) => write!(f, "{}", e),
            _ => write!(f, "Unknown error {}", self.to_owned())
        }
    }
}