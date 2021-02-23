use anyhow::Result;
use serde::Deserialize;

use slack_client::client::SlackApiClient;
use slack_client::requests::AuthTestRequest;

#[derive(Debug, Deserialize)]
struct AuthTest {
    ok: bool,
    user: String,
    user_id: String,
    team: String,
    team_id: String,
    bot_id: String,
    url: String,
    is_enterprise_install: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = AuthTestRequest::new();

    let mut response = client.request(&request).await?;
    let body = response.body_mut();
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;
    let auth_test: AuthTest = serde_json::from_slice(bytes.as_ref())?;
    println!("{:?}", auth_test);

    Ok(())
}
