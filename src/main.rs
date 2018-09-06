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

fn show_response_body(body: &[u8]) {
    let json = parse_response_body(body);
    println!("{:?}", json);
}

fn parse_response_body(body: &[u8]) -> Result<Authenticated, serde_json::Error> {
    serde_json::from_slice::<Authenticated>(body)
}

fn show_response(response: Response<Body>) {
    let body = response.into_body();
    let _result = body.concat2().wait()
        .map(|payload| show_response_body(&payload.into_bytes()))
        .map_err(|error| println!("Failed to parse response body: {}", error));
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

fn main() {
    tokio::run(future::lazy(|| {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder()
            .build::<_, Body>(https);

        let token = find_token();
        let uri = create_authentication_uri(token);
        client.get(uri)
            .map(|response| show_response(response))
            .map_err(|error| println!("{:?}", error))
    }));
}
