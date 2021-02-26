use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthTest {
    ok: bool,
    user: String,
    user_id: String,
    team: String,
    team_id: String,
    bot_id: String,
    url: String,
    is_enterprise_install: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostMessage {
    channel: String,
    ts: String,
    message: BotMessage,
}

#[derive(Debug, Deserialize)]
pub struct BotMessage {
    text: String,
    username: String,
    bot_id: String,
    ts: String,
}

#[derive(Deserialize, Debug)]
pub struct Identity {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    domain: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct RtmConnect {
    #[serde(rename = "self")]
    identity: Identity,
    team: Team,
    pub url: String,
}
