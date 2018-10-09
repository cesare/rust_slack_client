extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

extern crate slack_client;

use futures::future;

use std::env;

use slack_client::client::*;
use slack_client::error::Error;
use slack_client::message::*;

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
