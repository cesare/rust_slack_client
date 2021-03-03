use anyhow::Result;
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use regex::Regex;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use slack_client::client::SlackApiClient;
use slack_client::events::Message;
use slack_client::requests::{PostMessageRequest, RtmConnectRequest};
use slack_client::responses::RtmConnect;

#[async_trait]
trait MessageHandler {
    fn matches(&self, text: &str) -> bool;
    async fn handle(&self, channel: &str, user: &str, text: &str) -> Result<()>;
}

struct PingMessageHandler {
    pattern: Regex,
}

impl PingMessageHandler {
    fn new() -> Self {
        PingMessageHandler {
            pattern: Regex::new(r"\bping\b").unwrap(),
        }
    }
}

#[async_trait]
impl MessageHandler for PingMessageHandler {
    fn matches(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }

    async fn handle(&self, channel: &str, user: &str, _text: &str) -> Result<()> {
        let client = SlackApiClient::new();
        let reply = format!("<@{}> pong", user);
        let request = PostMessageRequest::new(channel, &reply);
        client.request(&request).await?;
        Ok(())
    }
}

struct MessageListener {
    rx: Mutex<Receiver<Message>>,
}

impl MessageListener {
    fn new(rx: Receiver<Message>) -> Self {
        MessageListener {
            rx: Mutex::new(rx),
        }
    }

    async fn run(&self) {
        let mut rx = self.rx.lock().await;
        while let Some(msg) = rx.recv().await {
            println!("{:?}", msg);
            let _ = self.handle_message(&msg).await;
        }
    }

    async fn handle_message(&self, msg: &Message) -> Result<()> {
        match msg {
            Message::Message { channel, user, text, ..} => {
                let ping_handler = PingMessageHandler::new();
                if ping_handler.matches(&text) {
                    ping_handler.handle(&channel, &user, &text).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }
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

#[tokio::main]
async fn main() -> Result<()> {
    let client = SlackApiClient::new();
    let request = RtmConnectRequest::new();

    let rtm_connect: RtmConnect = client.request(&request).await?;
    println!("{:?}", rtm_connect);

    let (mut tx, rx) = channel::<Message>(100);
    let msg_handle = tokio::spawn(async move {
        MessageListener::new(rx).run().await;
    });

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    let ws_handle = tokio::spawn(async move {
        wait_for_messages(&mut stream, &mut tx).await
    });

    let (_, _) = tokio::join!(msg_handle, ws_handle);
    Ok(())
}
