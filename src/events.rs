use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "hello")]
    Hello {},

    #[serde(rename = "message")]
    Message {
        team: String,
        channel: String,
        user: String,
        user_team: String,
        text: String,
        ts: String,
    },

    #[serde(rename = "user_typing")]
    UserTyping {
        channel: String,
        user: String,
    }
}

pub struct MessageEvent {
    pub channel: String,
    pub user: String,
    pub text: String,
}

impl MessageEvent {
    pub fn new(channel: &str, user: &str, text: &str) -> Self {
        MessageEvent {
            channel: channel.to_owned(),
            user: user.to_owned(),
            text: text.to_owned(),
        }
    }
}
