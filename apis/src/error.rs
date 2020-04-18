use ::std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiriusError {
    pub code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum Error {
    SiriusError(SiriusError),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Failure(failure::Error),
}

impl ::failure::Fail for Error {}

#[derive(Debug)]
pub struct ApiError {
    pub code: reqwest::StatusCode,
    pub content: Option<String>,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        return Error::Reqwest(e);
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.to_owned() {
            Error::SiriusError(e) => {
                write!(f, "{{ code: \"{}\", message: \"{}\" }}", e.code, e.message)
            }
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Serde(e) => write!(f, "{}", e),
            Error::Failure(e) => write!(f, "{}", e),
        }
    }
}
