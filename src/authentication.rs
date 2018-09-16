use futures::Future;
use http::Response;
use hyper::{Body, Uri};
use hyper::rt::Stream;
use serde_json;
use url::form_urlencoded::Serializer;

use client::*;

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
pub struct Authenticated {
    pub ok: bool,
    #[serde(rename = "self")]
    pub identity: Identity,
    pub team: Team,
    pub url: String,
}

fn parse_response(response: Response<Body>) -> Result<Authenticated, Error> {
    let body = response.into_body().concat2().wait()?;
    serde_json::from_slice::<Authenticated>(&body.into_bytes()).map_err(|_e| Error::ParseJsonFailed)
}

fn create_query_string(token: &String) -> String {
    Serializer::new(String::new()).append_pair("token", token).finish()
}

fn create_authentication_uri(token: String) -> Uri {
    let query = create_query_string(&token);
    let url_string = format!("https://slack.com/api/rtm.connect?{}", query);
    url_string.parse::<Uri>().unwrap()
}

pub fn authenticate(client: &HttpClient) -> Result<Authenticated, Error> {
    let token = find_token()?;
    let uri = create_authentication_uri(token);
    let response = client.get(uri).wait()?;
    parse_response(response)
}
