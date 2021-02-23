use anyhow::Result;
use hyper_tls::HttpsConnector;
use hyper::{Body, Client, Response};
use hyper::client::HttpConnector;

use crate::requests::SlackApiRequest;

pub fn create_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

pub struct SlackApiClient {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl SlackApiClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let http_client = Client::builder().build::<_, hyper::Body>(https);
        SlackApiClient {
            http_client: http_client,
        }
    }

    pub async fn request<T>(&self, request: &T) -> Result<Response<Body>>  where T: SlackApiRequest {
        let http_request = request.build()?;
        let response = self.http_client.request(http_request).await?;
        Ok(response)
    }
}
