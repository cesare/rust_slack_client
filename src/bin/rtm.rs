extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

extern crate slack_client;

use futures::{future, Future};
use hyper::{Body, Request};
use hyper::header::{CONNECTION, UPGRADE};

use slack_client::authentication::*;
use slack_client::client::*;

fn connect_websocket(url: &String) -> Result<(), Error> {
    println!("connecting: {}", url);
    let client = create_client()?;
    let request = Request::builder()
        .uri(url)
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "Upgrade")
        .body(Body::empty())
        .unwrap();

    client.request(request)
        .and_then(|response| {
            println!("{:?}", response);
            response.into_body().on_upgrade()
        })
        .map(|upgraded| {
            let _ = tokio::io::read_to_end(upgraded, Vec::new())
                .map(|(_upgraded, vec)| println!("{:?}", std::str::from_utf8(&vec)));
        })
        .map_err(|_error| Error::HttpFailed)
        .wait()
}

fn start() -> Result<(), Error> {
    let client = SlackApiClient::create()?;
    let response = authenticate(&client)?;
    connect_websocket(&response.body.url)
}

fn main() {
    tokio::run(future::lazy(|| {
        start().map_err(|error| println!("{:?}", error))
    }));
}
