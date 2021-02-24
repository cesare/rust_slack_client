use anyhow::Result;

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

    let json = client.request(&request).await?;
    println!("{:?}", json);

    Ok(())
}
