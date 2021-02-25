use anyhow::Result;
use hyper::{Body, Request};

pub trait SlackApiRequest {
    fn build(self: &Self) -> Result<Request<Body>>;
}

pub struct AuthTestRequest {
}

impl AuthTestRequest {
    pub fn new() -> Self {
        AuthTestRequest {}
    }
}

impl SlackApiRequest for AuthTestRequest {
    fn build(self: &Self) -> Result<Request<Body>> {
        let slack_token = std::env::var("SLACK_TOKEN")?;

        let request = Request::builder()
            .method("POST")
            .uri("https://slack.com/api/auth.test")
            .header("Authorization", format!("Bearer {}", slack_token))
            .body(Body::empty())?;
        Ok(request)
    }
}

pub struct PostMessageRequest {
    channel: String,
    text: String,
}

impl PostMessageRequest {
    pub fn new(channel: &str, text: &str) -> Self {
        PostMessageRequest {
            channel: channel.to_owned(),
            text: text.to_owned(),
        }
    }
}

impl SlackApiRequest for PostMessageRequest {
    fn build(self: &Self) -> Result<Request<Body>> {
        let slack_token = std::env::var("SLACK_TOKEN")?;

        let query = form_urlencoded::Serializer::new(String::new())
            .append_pair("channel", &self.channel)
            .append_pair("text", &self.text)
            .finish();

        let request = Request::builder()
            .method("POST")
            .uri("https://slack.com/api/chat.postMessage")
            .header("Authorization", format!("Bearer {}", slack_token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(query.into())?;
        Ok(request)
    }
}

pub struct RtmConnectRequest {
}

impl RtmConnectRequest {
    pub fn new() -> Self {
        RtmConnectRequest {}
    }
}

impl SlackApiRequest for RtmConnectRequest {
    fn build(&self) -> Result<Request<Body>> {
        let slack_token = std::env::var("SLACK_TOKEN")?;

        let query = form_urlencoded::Serializer::new(String::new())
            .append_pair("batch_presence_aware", "1")
            .append_pair("presence_sub", "1")
            .finish();

        let request = Request::builder()
            .method("POST")
            .uri("https://slack.com/api/rtm.connect")
            .header("Authorization", format!("Bearer {}", slack_token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(query.into())?;
        Ok(request)
    }
}
