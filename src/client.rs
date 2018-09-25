use futures::Future;
use hyper;
use hyper::{Body, Client, Request, Response};
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

pub trait SlackApiRequest {
    fn path(&self) -> String;
}

pub trait SlackApiPostRequest: SlackApiRequest {
    fn body(&self) -> String;
}

pub trait SlackApiResponse {

}

pub struct SlackApiClient {
    http_client: HttpClient,
}

impl SlackApiClient {
    pub fn create() -> Result<SlackApiClient, Error> {
        let http_client = create_client()?;

        let client = SlackApiClient {
            http_client: http_client,
        };
        Ok(client)
    }

    pub fn post<T>(&self, request: &T) -> Result<Response<Body>, Error>
        where T: SlackApiPostRequest {
        let uri = self.create_uri(request);
        let query = request.body();
        let req = Request::post(uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(query))
            .unwrap();
        self.http_client.request(req)
            .map_err(|_e| Error::HttpFailed)
            .wait()
    }

    fn create_uri<R: SlackApiRequest>(&self, request: &R) -> String {
        let path = request.path();
        format!("https://slack.com/{}", path)
    }
}
