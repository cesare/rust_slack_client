use anyhow::Result;

use slack_client::client::SlackApiClient;
use slack_client::requests::AuthTestRequest;
use slack_client::responses::AuthTest;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = AuthTestRequest::new();

    let json = client.request(&request).await?;
    let auth_test: AuthTest = serde_json::from_value(json)?;
    println!("{:?}", auth_test);

    Ok(())
}
