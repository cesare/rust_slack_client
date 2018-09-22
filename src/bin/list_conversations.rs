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
use hyper::{Body, Uri};
use hyper::rt::Stream;
use url::form_urlencoded::Serializer;

use slack_client::client::*;

#[derive(Debug, Deserialize)]
struct ListChannelsResponse {
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
    client: HttpClient,
    token: String,
}

impl ListChannelsRequest {
    fn new() -> Result<ListChannelsRequest, Error> {
        let client = create_client()?;
        let token = find_token()?;

        let request = ListChannelsRequest {
            client: client,
            token: token,
        };
        Ok(request)
    }

    fn parse_response(&self, response: Response<Body>) -> Result<ListChannelsResponse, Error> {
        let body = response.into_body().concat2().wait()?;
        serde_json::from_slice::<ListChannelsResponse>(&body.into_bytes()).map_err(|_e| Error::ParseJsonFailed)
    }

    fn create_query_string(&self) -> String {
        Serializer::new(String::new()).append_pair("token", &self.token).finish()
    }

    fn create_request_uri(&self) -> Uri {
        let query = self.create_query_string();
        let url_string = format!("https://slack.com/api/conversations.list?{}", query);
        url_string.parse::<Uri>().unwrap()
    }

    fn send(&self) -> Result<ListChannelsResponse, Error> {
        let uri = self.create_request_uri();
        let response = self.client.get(uri).wait()?;
        self.parse_response(response)
    }
}

fn show_channel(ch: &Channel) {
    println!("{} {}", ch.id, ch.name);
}

fn start() -> Result<(), Error> {
    let request = ListChannelsRequest::new()?;
    let response = request.send()?;

    for ch in &response.channels {
        show_channel(ch);
    }
    Ok(())
}

fn main() {
    tokio::run(future::lazy(|| {
        start().map_err(|error| println!("{:?}", error))
    }));
}
