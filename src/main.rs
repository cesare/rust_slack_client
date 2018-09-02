extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use futures::{future, Future};
use hyper::Client;
use hyper_tls::HttpsConnector;

fn main() {
    tokio::run(future::lazy(|| {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        client.get("https://slack.com".parse().unwrap())
            .map(|response| println!("{:?}", response))
            .map_err(|error| println!("{:?}", error))
    }));
}
