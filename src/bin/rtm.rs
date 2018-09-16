extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;
extern crate url;

#[macro_use]
extern crate serde_derive;

extern crate slack_client;

use futures::{future, Future};
use http::Response;
use hyper::{Body, Request, Uri};
use hyper::header::{CONNECTION, UPGRADE};
use hyper::rt::Stream;
use url::form_urlencoded::Serializer;

use slack_client::client::*;

#[derive(Deserialize, Debug)]
struct Identity {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Team {
    domain: String,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Authenticated {
    ok: bool,
    #[serde(rename = "self")]
    identity: Identity,
    team: Team,
    url: String,
}

fn connect_websocket(url: &String) -> Result<(), Error> {
    println!("connecting: {}", url);
    let client = create_client()?;
    let request = Request::builder()
        .uri(url)
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "Upgrade")
        .body(Body::empty())
        .unwrap();

    client.request(request)
        .and_then(|response| {
            println!("{:?}", response);
            response.into_body().on_upgrade()
        })
        .map(|upgraded| {
            let _ = tokio::io::read_to_end(upgraded, Vec::new())
                .map(|(_upgraded, vec)| println!("{:?}", std::str::from_utf8(&vec)));
        })
        .map_err(|_error| Error::HttpFailed)
        .wait()
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

fn authenticate(client: &HttpClient) -> Result<Authenticated, Error> {
    let token = find_token()?;
    let uri = create_authentication_uri(token);
    let response = client.get(uri).wait()?;
    parse_response(response)
}

fn start() -> Result<(), Error> {
    let client = create_client()?;
    let authenticated = authenticate(&client)?;
    connect_websocket(&authenticated.url)
}

fn main() {
    tokio::run(future::lazy(|| {
        start().map_err(|error| println!("{:?}", error))
    }));
}
