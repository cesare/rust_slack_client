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
