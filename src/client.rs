use hyper;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use serde_json;

use std::env;

#[derive(Debug)]
pub enum Error {
    TokenMissing,
    ParseJsonFailed,
    HttpFailed,
}

impl From<serde_json::Error> for Error {
    fn from(_e: serde_json::Error) -> Self {
        Error::ParseJsonFailed
    }
}

impl From<hyper::Error> for Error {
    fn from(_e: hyper::Error) -> Self {
        Error::HttpFailed
    }
}


pub type HttpClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

pub fn find_token() -> Result<String, Error> {
    env::var("SLACK_TOKEN")
        .map(|value| value.clone())
        .map_err(|_e| Error::TokenMissing)
}

pub fn create_client() -> Result<HttpClient, Error> {
    HttpsConnector::new(4)
        .map(|https| Client::builder().build::<_, Body>(https))
        .map_err(|_error| Error::HttpFailed)
}