use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Topic {
    value: String,
    creator: String,
    last_set: u32,
}

#[derive(Debug, Deserialize)]
struct Purpose {
    value: String,
    creator: String,
    last_set: u32,
}

#[derive(Debug, Deserialize)]
struct Channel {
    id: String,
    name: String,
    num_members: u32,
    is_private: bool,
    topic: Topic,
    purpose: Purpose,
}

#[derive(Debug, Deserialize)]
struct Channels {
    channels: Vec<Channel>,
}

fn create_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

fn create_request(slack_token: &str) -> Result<Request<Body>, hyper::http::Error> {
    Request::builder()
        .method("GET")
        .uri("https://slack.com/api/conversations.list")
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
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;

    let json: Value = serde_json::from_slice(bytes.as_ref())?;
    let channels: Channels = serde_json::from_value(json)?;
    println!("{:?}", channels);

    Ok(())
}
