extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

extern crate slack_client;

use futures::future;

use slack_client::client::*;
use slack_client::conversation::*;
use slack_client::error::Error;


fn show_channel(ch: &Channel) {
    println!("{} {}", ch.id, ch.name);
}

fn start() -> Result<(), Error> {
    let client = SlackApiClient::create()?;
    let request = ListChannelsRequest::new();
    let response: ListChannelsResponse = client.get(&request)?;

    for ch in &response.body.channels {
        show_channel(ch);
    }
    Ok(())
}

fn main() {
    tokio::run(future::lazy(|| {
        start().map_err(|error| println!("{:?}", error))
    }));
}
