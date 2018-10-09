use futures::Async;
use futures::Future;
use http::Response;
use hyper::Body;
use hyper::rt::Stream;
use serde_json::from_slice;

use client::*;
use error::Error;

pub struct ListChannelsResponse {
    pub body: ListChannels,
}

impl SlackApiResponse for ListChannelsResponse {
    fn create(response: Response<Body>) -> Result<Self, Error> {
        let body = response.into_body().concat2().wait()?;
        let parsed = from_slice::<ListChannels>(&body.into_bytes())?;
        let result = ListChannelsResponse {
            body:parsed,
        };
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
pub struct ListChannels {
    pub ok: bool,
    pub channels: Vec<Channel>,
    pub response_metadata: ResponseMetadata,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}

pub struct ListChannelsStream {
    client: HttpClient,
}

impl ListChannelsStream {
    pub fn new(client: HttpClient) -> ListChannelsStream {
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

pub struct ListChannelsRequest {
}

impl ListChannelsRequest {
    pub fn new() -> ListChannelsRequest {
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
