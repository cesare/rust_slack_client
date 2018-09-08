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

fn show_response(response: Response<Body>) {
    let _ = parse_response(response)
        .map(|body| println!("{:?}", body))
        .map_err(|error| println!("{:?}", error));
}

fn create_query_string(token: &String) -> String {
    Serializer::new(String::new()).append_pair("token", token).finish()
}

fn create_authentication_uri(token: String) -> Uri {
    let query = create_query_string(&token);
    let url_string = format!("https://slack.com/api/rtm.connect?{}", query);
    url_string.parse::<Uri>().unwrap()
}

fn find_token() -> String {
    env::var("SLACK_TOKEN").unwrap_or_else(|_error| panic!("SLACK_TOKEN missing")).clone()
}

type HttpClient = Client<HttpsConnector<hyper::client::HttpConnector>>;
fn create_client() -> HttpClient {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    Client::builder().build::<_, Body>(https)
}

fn main() {
    tokio::run(future::lazy(|| {
        let client = create_client();
        let token = find_token();
        let uri = create_authentication_uri(token);
        client.get(uri)
            .map(|response| show_response(response))
            .map_err(|error| println!("{:?}", error))
    }));
}
