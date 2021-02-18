use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "hello")]
    Hello {},

    #[serde(rename = "user_typing")]
    UserTyping {
        channel: String,
        user: String,
    }
}
