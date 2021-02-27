use anyhow::Result;

use slack_client::client::SlackApiClient;
use slack_client::requests::ListConversationsRequest;
use slack_client::responses::Channels;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = ListConversationsRequest::new();
    let channels: Channels = client.request(&request).await?;
    println!("{:?}", channels);

    Ok(())
}
