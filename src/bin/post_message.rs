use anyhow::Result;

use std::env;

use slack_client::client::SlackApiClient;
use slack_client::requests::PostMessageRequest;
use slack_client::responses::PostMessage;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let channel = &args[1];
    let text = &args[2];

    let client = SlackApiClient::new();
    let request = PostMessageRequest::new(channel, text);

    let post_message: PostMessage = client.request(&request).await?;
    println!("{:?}", post_message);

    Ok(())
}
