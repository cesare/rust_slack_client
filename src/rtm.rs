use futures_util::stream::StreamExt;
use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;
use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

mod events;

#[derive(Deserialize, Debug)]
struct Identity {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Team {
    domain: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct RtmConnect {
    #[serde(rename = "self")]
    identity: Identity,
    team: Team,
    url: String,
}

fn create_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

fn create_request(slack_token: &str) -> Result<Request<Body>, hyper::http::Error> {
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("batch_presence_aware", "1")
        .append_pair("presence_sub", "1")
        .finish();

    Request::builder()
        .method("POST")
        .uri("https://slack.com/api/rtm.connect")
        .header("Authorization", format!("Bearer {}", slack_token))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query.into())
}

#[tokio:: main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let slack_token = std::env::var("SLACK_TOKEN")?;

    let client = create_client();
    let request = create_request(&slack_token)?;

    let mut response = client.request(request).await?;
    let body = response.body_mut();
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;

    let rtm_connect: RtmConnect = serde_json::from_slice(bytes.as_ref())?;
    println!("{:?}", rtm_connect);

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    while let Some(Ok(message)) = stream.next().await {
        match message {
            Message::Text(text) => {
                let json: Value = serde_json::from_str(&text)?;
                let original_json = json.clone();
                if let Ok(msg) = serde_json::from_value::<events::Message>(json) {
                    println!("{:?}", msg);
                } else {
                    println!("{}", original_json);
                }
            }
            _ => {
                println!("Non-text message: {:?}", message);
            }
        }
    }

    Ok(())
}