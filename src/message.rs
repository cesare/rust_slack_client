use futures::Future;
use http::Response;
use hyper::Body;
use hyper::rt::Stream;
use serde_json::from_slice;

use client::*;
use error::Error;

pub struct PostMessageRequest {
    channel: String,
    text: String,
}

impl PostMessageRequest {
    pub fn new(channel: String, text: String) -> PostMessageRequest {
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

pub struct PostMessageResponse {
    pub body: PostMessage,
}

impl SlackApiResponse for PostMessageResponse {
    fn create(response: Response<Body>) -> Result<Self, Error> {
        let body = response.into_body().concat2().wait()?;
        let parsed = from_slice::<PostMessage>(&body.into_bytes())?;
        let result = PostMessageResponse {
            body: parsed,
        };
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostMessage {
    pub ok: bool,
    pub error: Option<String>,
}
