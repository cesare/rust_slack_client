use anyhow::Result;
use futures::stream::{Stream, StreamExt};
use regex::Regex;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use slack_client::client::SlackApiClient;
use slack_client::events::{Event, Message};
use slack_client::requests::{PostMessageRequest, RtmConnectRequest};
use slack_client::responses::RtmConnect;

trait MessageHandler {
    fn matches(&self, message: &Message) -> bool;
    fn handle(&self, message: &Message) -> Result<()>;
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

    fn matches(&self, message: &Message) -> bool {
        self.pattern.is_match(&message.text)
    }

    async fn handle(&self, message: &Message) -> Result<()> {
        let client = SlackApiClient::new();
        let reply = format!("<@{}> pong", message.user);
        let request = PostMessageRequest::new(&message.channel, &reply);
        client.request(&request).await?;
        Ok(())
    }
}

struct EventListener {
    rx: Mutex<Receiver<Event>>,
    ping_handler: PingMessageHandler,
}

impl EventListener {
    fn new(rx: Receiver<Event>) -> Self {
        EventListener {
            rx: Mutex::new(rx),
            ping_handler: PingMessageHandler::new(),
        }
    }

    async fn run(&self) {
        let mut rx = self.rx.lock().await;
        while let Some(event) = rx.recv().await {
            println!("{:?}", event);
            let _ = self.handle_event(&event).await;
        }
    }

    async fn handle_event(&self, event: &Event) -> Result<()> {
        match event {
            Event::Message { channel, user, text, ..} => {
                let message = Message::new(channel, user, text);
                if self.ping_handler.matches(&message) {
                    self.ping_handler.handle(&message).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

async fn wait_for_messages<S>(stream: &mut S, tx: &mut Sender<Event>)
    where S: Stream<Item = Result<WsMessage, WsError>> + Unpin + Send
{
    while let Some(Ok(message)) = stream.next().await {
        if let WsMessage::Text(text) = message {
            if let Ok(event) = serde_json::from_str::<Event>(&text) {
                let _ = tx.send(event).await;
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

    let (mut tx, rx) = channel::<Event>(100);
    let msg_handle = tokio::spawn(async move {
        EventListener::new(rx).run().await;
    });

    let (mut stream, _response) = tokio_tungstenite::connect_async(rtm_connect.url).await?;
    let ws_handle = tokio::spawn(async move {
        wait_for_messages(&mut stream, &mut tx).await
    });

    let (_, _) = tokio::join!(msg_handle, ws_handle);
    Ok(())
}
