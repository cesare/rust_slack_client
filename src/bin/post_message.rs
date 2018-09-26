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
use hyper::Body;
use hyper::rt::Stream;
use url::form_urlencoded::Serializer;

use std::env;

use slack_client::client::*;

struct PostMessageRequest {
    channel: String,
    text: String,
}

impl PostMessageRequest {
    fn new(channel: String, text: String) -> PostMessageRequest {
        PostMessageRequest {
            channel: channel,
            text: text,
        }
    }
}

impl SlackApiRequest for PostMessageRequest {
    fn path(&self) -> String {
        "api/chat.postMessage".to_string()
    }
}

impl SlackApiPostRequest for PostMessageRequest {
    fn body(&self) -> Result<String, Error> {
        let token = self.find_token()?;
        let body = Serializer::new(String::new())
            .append_pair("token", &token)
            .append_pair("channel", &self.channel)
            .append_pair("text", &self.text)
            .finish();
        Ok(body)
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

fn post_request(client: SlackApiClient, request: PostMessageRequest) -> Result<(), Error> {
    client.post(&request)
        .map(|response| {
            let response_body = parse_response(response).unwrap();
            println!("{:?}", response_body)
        })
}

fn start(channel: String, text: String) -> Result<(), Error> {
    let client = SlackApiClient::create()?;
    let request = PostMessageRequest::new(channel, text);
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
