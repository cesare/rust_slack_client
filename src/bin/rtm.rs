use anyhow::Result;
use futures::stream::{Stream, TryStreamExt};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;

use slack_client::client::SlackApiClient;
use slack_client::events;
use slack_client::requests::RtmConnectRequest;
use slack_client::responses::RtmConnect;

async fn wait_for_events(stream: &mut (dyn Stream<Item = Result<Message, WsError>> + Unpin)) -> Result<()> {
    while let Some(message) = stream.try_next().await? {
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

#[tokio:: main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = RtmConnectRequest::new();

    let rtm_connect: RtmConnect = client.request(&request).await?;
    println!("{:?}", rtm_connect);

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    wait_for_events(&mut stream).await?;

    Ok(())
}
