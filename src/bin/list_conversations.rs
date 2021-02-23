use anyhow::Result;
use hyper::{Body, Request};
use serde::Deserialize;
use serde_json::Value;

use slack_client::client;

#[derive(Debug, Deserialize)]
struct Topic {
    value: String,
    creator: String,
    last_set: u32,
}

#[derive(Debug, Deserialize)]
struct Purpose {
    value: String,
    creator: String,
    last_set: u32,
}

#[derive(Debug, Deserialize)]
struct Channel {
    id: String,
    name: String,
    num_members: Option<u32>,
    is_private: bool,
    topic: Topic,
    purpose: Purpose,
}

#[derive(Debug, Deserialize)]
struct Channels {
    channels: Vec<Channel>,
}

fn create_request(slack_token: &str) -> Result<Request<Body>> {
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("types", "public_channel,private_channel")
        .finish();

    let request = Request::builder()
        .method("POST")
        .uri("https://slack.com/api/conversations.list")
        .header("Authorization", format!("Bearer {}", slack_token))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query.into())?;
    Ok(request)
}

#[tokio::main]
async fn main() -> Result<()> {
    let slack_token = std::env::var("SLACK_TOKEN")?;

    let client = client::create_client();
    let request = create_request(&slack_token)?;

    let mut response = client.request(request).await?;
    let body = response.body_mut();
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;

    let json: Value = serde_json::from_slice(bytes.as_ref())?;
    let channels: Channels = serde_json::from_value(json)?;
    println!("{:?}", channels);

    Ok(())
}
