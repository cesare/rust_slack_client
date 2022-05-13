use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthTest {
    pub ok: bool,
    pub user: String,
    pub user_id: String,
    pub team: String,
    pub team_id: String,
    pub bot_id: String,
    pub url: String,
    pub is_enterprise_install: bool,
}

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub value: String,
    pub creator: String,
    pub last_set: u32,
}

#[derive(Debug, Deserialize)]
pub struct Purpose {
    pub value: String,
    pub creator: String,
    pub last_set: u32,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub num_members: Option<u32>,
    pub is_private: bool,
    pub topic: Topic,
    pub purpose: Purpose,
}

#[derive(Debug, Deserialize)]
pub struct Channels {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Deserialize)]
pub struct PostMessage {
    pub channel: String,
    pub ts: String,
    pub message: BotMessage,
}

#[derive(Debug, Deserialize)]
pub struct BotMessage {
    pub text: String,
    pub username: String,
    pub bot_id: String,
    pub ts: String,
}

#[derive(Deserialize, Debug)]
pub struct Identity {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    pub domain: String,
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct RtmConnect {
    #[serde(rename = "self")]
    pub identity: Identity,
    pub team: Team,
    pub url: String,
}
