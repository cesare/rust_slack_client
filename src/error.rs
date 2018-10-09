use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    TokenMissing,
    ParseJsonFailed(String),
    HttpFailed(String),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseJsonFailed(format!("{}", e))
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::HttpFailed(format!("{}", e))
    }
}
