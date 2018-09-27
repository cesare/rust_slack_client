extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

extern crate slack_client;

use futures::future;

use slack_client::authentication::*;
use slack_client::client::*;


fn start() -> Result<(), Error> {
    let client = SlackApiClient::create()?;
    let authenticated = authenticate(&client)?;
    println!("{:?}", authenticated);
    Ok(())
}

fn main() {
    tokio::run(future::lazy(|| {
        start().map_err(|error| println!("{:?}", error))
    }));
}
