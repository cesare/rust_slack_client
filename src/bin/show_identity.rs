use anyhow::Result;
use hyper::{Body, Request};
use serde::Deserialize;

use slack_client::client;

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

fn create_request(slack_token: &str) -> Result<Request<Body>> {
    let request = Request::builder()
        .method("POST")
        .uri("https://slack.com/api/auth.test")
        .header("Authorization", format!("Bearer {}", slack_token))
        .body(Body::empty())?;
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
    let auth_test: AuthTest = serde_json::from_slice(bytes.as_ref())?;
    println!("{:?}", auth_test);

    Ok(())
}
