use hyper_tls::HttpsConnector;
use hyper::{Body, Client};
use hyper::client::HttpConnector;

pub fn create_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}
