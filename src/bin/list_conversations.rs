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
use futures::Async;
use http::Response;
use hyper::Body;
use hyper::rt::Stream;

use slack_client::client::*;
use slack_client::error::Error;

struct ListChannelsResponse {
    body: ListChannels,
}

impl SlackApiResponse for ListChannelsResponse {
    fn create(response: Response<Body>) -> Result<Self, Error> {
        let body = response.into_body().concat2().wait()?;
        let parsed = serde_json::from_slice::<ListChannels>(&body.into_bytes())?;
        let result = ListChannelsResponse {
            body:parsed,
        };
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct ListChannels {
    ok: bool,
    channels: Vec<Channel>,
    response_metadata: ResponseMetadata,
}

#[derive(Debug, Deserialize)]
struct Channel {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct ResponseMetadata {
    next_cursor: String,
}

struct ListChannelsStream {
    client: HttpClient,
}

impl ListChannelsStream {
    fn new(client: HttpClient) -> ListChannelsStream {
        ListChannelsStream {
            client: client,
        }
    }
}

impl Stream for ListChannelsStream {
    type Item = Channel;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Channel>>, Error> {
        Ok(Async::NotReady)
    }
}

struct ListChannelsRequest {
}

impl ListChannelsRequest {
    fn new() -> ListChannelsRequest {
        ListChannelsRequest {
        }
    }
}

impl SlackApiRequest for ListChannelsRequest {
    fn path(&self) -> String {
        "api/conversations.list".to_string()
    }

    fn query_string(&self) -> Result<Option<String>, Error> {
        let empty_params: Vec<(String, String)> = vec![];
        let query = self.create_query_string(empty_params)?;
        Ok(Some(query))
    }
}

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
