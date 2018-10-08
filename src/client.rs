use futures::Future;
use hyper;
use hyper::{Body, Client, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use serde_json;
use url::form_urlencoded::Serializer;

use std::borrow::Borrow;
use std::env;
use std::marker::Sized;

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


pub type HttpClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

pub fn create_client() -> Result<HttpClient, Error> {
    HttpsConnector::new(4)
        .map(|https| Client::builder().build::<_, Body>(https))
        .map_err(|e| Error::HttpFailed(format!("{}", e)))
}

pub trait SlackApiRequest {
    fn path(&self) -> String;

    fn query_string(&self) -> Result<Option<String>, Error> {
        Ok(None)
    }

    fn find_token(&self) -> Result<String, Error> {
        env::var("SLACK_TOKEN")
            .map(|value| value.clone())
            .map_err(|_e| Error::TokenMissing)
    }

    fn create_query_string<I, K, V>(&self, params: I) -> Result<String, Error>
        where I: IntoIterator, I::Item: Borrow<(K, V)>, K: AsRef<str>, V: AsRef<str>
    {
        let token = self.find_token()?;
        let query = Serializer::new(String::new())
            .append_pair("token", &token)
            .extend_pairs(params)
            .finish();
        Ok(query)
    }
}

pub trait SlackApiGetRequest: SlackApiRequest {
}

pub trait SlackApiPostRequest: SlackApiRequest {
    fn body(&self) -> Result<String, Error>;
}

pub trait SlackApiResponse {
    fn create(response: Response<Body>) -> Result<Self, Error> where Self: Sized;
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

    pub fn get<T, S>(&self, request: &T) -> Result<S, Error>
        where T: SlackApiRequest, S: SlackApiResponse
    {
        let uri = self.create_uri(request)?;
        let response = self.http_client.get(uri)
            .wait()?;
        S::create(response)
    }

    pub fn post<T, S>(&self, request: &T) -> Result<S, Error>
        where T: SlackApiPostRequest, S: SlackApiResponse {
        let uri = self.create_uri(request)?;
        let query = request.body()?;
        let req = Request::post(uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from(query))
            .unwrap();
        let response = self.http_client.request(req)
            .wait()?;
        S::create(response)
    }

    fn create_uri<R: SlackApiRequest>(&self, request: &R) -> Result<Uri, Error> {
        let path = request.path();
        let query_string = request.query_string()?;
        let uri_string = query_string.map_or_else(
            || format!("https://slack.com/{}", path),
            |query| format!("https://slack.com/{}?{}", path, query)
        );
        let uri = uri_string.parse::<Uri>().unwrap();
        Ok(uri)
    }
}
