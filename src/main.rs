extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

#[macro_use]
extern crate serde_derive;

use std::env;

use futures::{future, Future};
use http::Response;
use hyper::{Body, Client, Uri};
use hyper::rt::Stream;
use hyper_tls::HttpsConnector;
use url::form_urlencoded::Serializer;

#[derive(Deserialize, Debug)]
struct Identity {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Team {
    domain: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Authenticated {
    ok: bool,
    #[serde(rename = "self")]
    identity: Identity,
    team: Team,
    url: String,
}

#[derive(Debug)]
struct Error {
}

impl From<serde_json::Error> for Error {
    fn from(_e: serde_json::Error) -> Self {
        Error {}
    }
}

impl From<hyper::Error> for Error {
    fn from(_e: hyper::Error) -> Self {
        Error {}
    }
}

fn parse_response(response: Response<Body>) -> Result<Authenticated, Error> {
    let body = response.into_body().concat2().wait()?;
    serde_json::from_slice::<Authenticated>(&body.into_bytes()).map_err(|e| Error::from(e))
}

fn create_query_string(token: &String) -> String {
    Serializer::new(String::new()).append_pair("token", token).finish()
}

fn create_authentication_uri(token: String) -> Uri {
    let query = create_query_string(&token);
    let url_string = format!("https://slack.com/api/rtm.connect?{}", query);
    url_string.parse::<Uri>().unwrap()
}

fn find_token() -> Result<String, Error> {
    env::var("SLACK_TOKEN")
        .map(|value| value.clone())
        .map_err(|_e| Error {})
}

type HttpClient = Client<HttpsConnector<hyper::client::HttpConnector>>;
fn create_client() -> Result<HttpClient, Error> {
    HttpsConnector::new(4)
        .map(|https| Client::builder().build::<_, Body>(https))
        .map_err(|_error| Error {})
}

fn authenticate() -> Result<Authenticated, Error> {
    let client = create_client()?;
    let token = find_token()?;
    let uri = create_authentication_uri(token);
    let response = client.get(uri).wait()?;
    parse_response(response)
}

fn main() {
    tokio::run(future::lazy(|| {
        authenticate()
            .map(|authenticated| println!("{:?}", authenticated))
            .map_err(|error| println!("{:?}", error))
    }));
}
