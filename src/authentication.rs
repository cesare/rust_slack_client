use futures::Future;
use http::Response;
use hyper::Body;
use hyper::rt::Stream;
use serde_json;

use client::*;

pub struct AuthenticationRequest {
}

impl AuthenticationRequest {
    pub fn new() -> AuthenticationRequest {
        AuthenticationRequest {}
    }
}

impl SlackApiRequest for AuthenticationRequest {
    fn path(&self) -> String {
        "api/rtm.connect".to_string()
    }

    fn query_string(&self) -> Result<Option<String>, Error> {
        let empty_params: Vec<(String, String)> = vec![];
        let query = self.create_query_string(empty_params)?;
        Ok(Some(query))
    }
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

pub fn authenticate(client: &SlackApiClient) -> Result<Authenticated, Error> {
    let request = AuthenticationRequest::new();
    let response = client.get(&request)?;
    parse_response(response)
}
