extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

use std::env;

use futures::{future, Future};
use hyper::{Body, Client, Uri};
use hyper::rt::Stream;
use hyper_tls::HttpsConnector;
use serde_json::{Value, Error};
use url::form_urlencoded::Serializer;

fn show_response_body(body: Body) {
    let result = body.concat2().wait();
    match result {
        Ok(payload) => {
            let json: Result<Value, Error> = serde_json::from_slice(&payload.into_bytes());
            println!("{:?}", json)
        }
        _ => println!("Failed to parse response body")
    }
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
            .map(|response| show_response_body(response.into_body()))
            .map_err(|error| println!("{:?}", error))
    }));
}
