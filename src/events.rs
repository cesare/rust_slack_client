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
