use futures::stream::StreamExt;
use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use tokio::io::AsyncWriteExt;

fn create_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

fn create_request(slack_token: &str) -> Result<Request<Body>, hyper::http::Error> {
    Request::builder()
        .method("POST")
        .uri("https://slack.com/api/auth.test")
        .header("Authorization", format!("Bearer {}", slack_token))
        .body(Body::empty())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack_token = std::env::var("SLACK_TOKEN")?;

    let client = create_client();
    let request = create_request(&slack_token)?;

    let mut response = client.request(request).await?;
    let body = response.body_mut();
    while let Some(bytes) = body.next().await {
        tokio::io::stdout().write_all(&bytes?).await?;
    }

    Ok(())
}
