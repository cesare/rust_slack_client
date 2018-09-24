extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

#[macro_use]
extern crate serde_derive;

extern crate slack_client;

use futures::future;
use futures::Future;
use http::Response;
use hyper::{Body, Request};
use hyper::rt::Stream;
use url::form_urlencoded::Serializer;

use std::env;

use slack_client::client::*;

struct PostMessageRequest {
    token: String,
    channel: String,
    text: String,
}

impl PostMessageRequest {
    fn new(token: String, channel: String, text: String) -> PostMessageRequest {
        PostMessageRequest {
            token: token,
            channel: channel,
            text: text,
        }
    }

    fn create_request_uri(&self) -> String {
        "https://slack.com/api/chat.postMessage".to_string()
    }

    fn create_request_body(&self) -> String {
        Serializer::new(String::new())
            .append_pair("token", &self.token)
            .append_pair("channel", &self.channel)
            .append_pair("text", &self.text)
            .finish()
    }
}

impl SlackApiRequest for PostMessageRequest {
    fn path(&self) -> String {
        "api/chat.postMessage".to_string()
    }
}

impl SlackApiPostRequest for PostMessageRequest {
    fn body(&self) -> String {
        Serializer::new(String::new())
            .append_pair("token", &self.token)
            .append_pair("channel", &self.channel)
            .append_pair("text", &self.text)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct PostMessageResponse {
    ok: bool,
    error: Option<String>,
}

fn parse_response(response: Response<Body>) -> Result<PostMessageResponse, Error> {
    let body = response.into_body().concat2().wait()?;
    serde_json::from_slice::<PostMessageResponse>(&body.into_bytes()).map_err(|_e| Error::ParseJsonFailed)
}

fn post_request(client: HttpClient, request: PostMessageRequest) -> Result<(), Error> {
    let uri = request.create_request_uri();
    let query = request.create_request_body();
    let req = Request::post(uri)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(Body::from(query))
        .unwrap();

    client.request(req)
        .map(|response| {
            let response_body = parse_response(response).unwrap();
            println!("{:?}", response_body)
        })
        .map_err(|_error| Error::HttpFailed)
        .wait()
}

fn start(channel: String, text: String) -> Result<(), Error> {
    let token = find_token()?;
    let client = create_client()?;
    let request = PostMessageRequest::new(token, channel, text);
    post_request(client, request)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let channel = args[1].to_string();
    let text = args[2].to_string();

    tokio::run(future::lazy(|| {
        start(channel, text).map_err(|error| println!("{:?}", error))
    }));
}
