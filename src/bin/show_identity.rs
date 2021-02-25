use anyhow::Result;

use slack_client::client::SlackApiClient;
use slack_client::requests::AuthTestRequest;
use slack_client::responses::AuthTest;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = AuthTestRequest::new();

    let auth_test: AuthTest = client.request(&request).await?;
    println!("{:?}", auth_test);

    Ok(())
}
