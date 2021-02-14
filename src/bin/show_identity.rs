use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack_token = std::env::var("SLACK_TOKEN")?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .method("POST")
        .uri("https://slack.com/api/auth.test")
        .header("Authorization", format!("Bearer {}", slack_token))
        .body(Body::empty())?;

    let response = client.request(request).await?;
    println!("{:?}", response);
    Ok(())
}
