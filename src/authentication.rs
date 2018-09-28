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

pub struct AuthenticationResponse {
    pub body: Authenticated,
}

impl SlackApiResponse for AuthenticationResponse {
    fn create(response: Response<Body>) -> Result<Self, Error> {
        let body = response.into_body().concat2().wait()?;
        let parsed = serde_json::from_slice::<Authenticated>(&body.into_bytes()).map_err(|_e| Error::ParseJsonFailed)?;
        let result = AuthenticationResponse {
            body:parsed,
        };
        Ok(result)
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

pub fn authenticate(client: &SlackApiClient) -> Result<AuthenticationResponse, Error> {
    let request = AuthenticationRequest::new();
    client.get2(&request)
}
