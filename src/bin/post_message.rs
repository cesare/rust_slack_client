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
        let params = vec![
            ("channel", &self.channel),
            ("text", &self.text),
        ];
        self.create_query_string(params)
    }
}

struct PostMessageResponse {
    body: PostMessage,
}

impl SlackApiResponse for PostMessageResponse {
    fn create(response: Response<Body>) -> Result<Self, Error> {
        let body = response.into_body().concat2().wait()?;
        let parsed = serde_json::from_slice::<PostMessage>(&body.into_bytes())?;
        let result = PostMessageResponse {
            body: parsed,
        };
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct PostMessage {
    ok: bool,
    error: Option<String>,
}

fn start(channel: String, text: String) -> Result<(), Error> {
    let client = SlackApiClient::create()?;
    let request = PostMessageRequest::new(channel, text);
    let response: PostMessageResponse = client.post(&request)?;
    println!("{:?}", response.body);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let channel = args[1].to_string();
    let text = args[2].to_string();

    tokio::run(future::lazy(|| {
        start(channel, text).map_err(|error| println!("{:?}", error))
    }));
}
