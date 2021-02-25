use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;

use slack_client::client::SlackApiClient;
use slack_client::events;
use slack_client::requests::RtmConnectRequest;
use slack_client::responses::RtmConnect;

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
    let client = SlackApiClient::new();
    let request = RtmConnectRequest::new();

    let rtm_connect: RtmConnect = client.request(&request).await?;
    println!("{:?}", rtm_connect);

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    wait_for_messages(&mut stream).await?;

    Ok(())
}
