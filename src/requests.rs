use anyhow::Result;
use hyper::{Body, Request};

pub trait SlackApiRequest {
    fn build(self: &Self) -> Result<Request<Body>>;
}
