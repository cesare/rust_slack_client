use hyper::{Body, Request};
use serde_json::Value;

use std::env;

mod client;

fn create_request(slack_token: &str, channel: &str, text: &str) -> Result<Request<Body>, hyper::http::Error> {
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("channel", channel)
        .append_pair("text", text)
        .finish();

    Request::builder()
        .method("POST")
        .uri("https://slack.com/api/chat.postMessage")
        .header("Authorization", format!("Bearer {}", slack_token))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query.into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let channel = &args[1];
    let text = &args[2];

    let slack_token = std::env::var("SLACK_TOKEN")?;

    let client = client::create_client();
    let request = create_request(&slack_token, channel, text)?;

    let mut response = client.request(request).await?;
    let body = response.body_mut();
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;

    let json: Value = serde_json::from_slice(bytes.as_ref())?;
    println!("{:?}", json);

    Ok(())
}
