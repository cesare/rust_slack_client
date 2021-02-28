use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use tokio::sync::mpsc::{channel, Sender};
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use slack_client::client::SlackApiClient;
use slack_client::events::Message;
use slack_client::requests::{PostMessageRequest, RtmConnectRequest};
use slack_client::responses::RtmConnect;

async fn handle_message(msg: &Message) -> Result<()> {
    match msg {
        Message::Message { channel, text, ..} => {
            if text == "ping" {
                let client = SlackApiClient::new();
                let request = PostMessageRequest::new(channel, "pong");
                client.request(&request).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

async fn wait_for_messages<S>(stream: &mut S, tx: &mut Sender<Message>)
    where S: Stream<Item = Result<WsMessage, WsError>> + Unpin + Send
{
    while let Some(Ok(message)) = stream.next().await {
        if let WsMessage::Text(text) = message {
            if let Ok(msg) = serde_json::from_str::<Message>(&text) {
                let _ = tx.send(msg).await;
            }
        }
    }
}

#[tokio:: main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = RtmConnectRequest::new();

    let rtm_connect: RtmConnect = client.request(&request).await?;
    println!("{:?}", rtm_connect);

    let (mut tx, mut rx) = channel::<Message>(100);

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    let handle = tokio::spawn(async move {
        wait_for_messages(&mut stream, &mut tx).await
    });

    while let Some(msg) = rx.recv().await {
        println!("{:?}", msg);
        let _ = handle_message(&msg).await;
    }

    handle.await?;
    Ok(())
}
