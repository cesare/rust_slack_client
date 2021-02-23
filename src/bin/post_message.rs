use anyhow::Result;
use serde_json::Value;

use std::env;

use slack_client::client::SlackApiClient;
use slack_client::requests::PostMessageRequest;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let channel = &args[1];
    let text = &args[2];

    let client = SlackApiClient::new();
    let request = PostMessageRequest::new(channel, text);

    let mut response = client.request(&request).await?;
    let body = response.body_mut();
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;

    let json: Value = serde_json::from_slice(bytes.as_ref())?;
    println!("{:?}", json);

    Ok(())
}
