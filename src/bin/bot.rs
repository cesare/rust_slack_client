use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use hyper::{Body, Request};
use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;

use slack_client::client;
use slack_client::events;

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

fn create_request(slack_token: &str) -> Result<Request<Body>> {
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("batch_presence_aware", "1")
        .append_pair("presence_sub", "1")
        .finish();

    let request = Request::builder()
        .method("POST")
        .uri("https://slack.com/api/rtm.connect")
        .header("Authorization", format!("Bearer {}", slack_token))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query.into())?;
    Ok(request)
}

async fn wait_for_messages(stream: &mut (dyn Stream<Item = Result<Message, WsError>> + Unpin)) -> Result<()> {
    while let Some(Ok(message)) = stream.next().await {
        if let Message::Text(text) = message {
            let json: Value = serde_json::from_str(&text)?;
            if let Ok(msg) = serde_json::from_value::<events::Message>(json) {
                println!("{:?}", msg);
            }
        }
    }

    Ok(())
}

#[tokio:: main]
async fn main() -> Result<()> {
    let slack_token = std::env::var("SLACK_TOKEN")?;

    let client = client::create_client();
    let request = create_request(&slack_token)?;

    let mut response = client.request(request).await?;
    let body = response.body_mut();
    let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;

    let rtm_connect: RtmConnect = serde_json::from_slice(bytes.as_ref())?;
    println!("{:?}", rtm_connect);

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    wait_for_messages(&mut stream).await?;

    Ok(())
}
