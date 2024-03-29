use anyhow::{anyhow, Result};
use hyper_tls::HttpsConnector;
use hyper::{Body, Client};
use hyper::client::HttpConnector;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::requests::SlackApiRequest;
use crate::responses::ErrorResponse;

pub fn create_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

pub struct SlackApiClient {
    http_client: Client<HttpsConnector<HttpConnector>, Body>,
}

impl Default for SlackApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl SlackApiClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let http_client = Client::builder().build::<_, hyper::Body>(https);
        SlackApiClient {
            http_client,
        }
    }

    pub async fn request<T, S>(&self, request: &T) -> Result<S>  where T: SlackApiRequest, S: DeserializeOwned {
        let http_request = request.build()?;
        let mut response = self.http_client.request(http_request).await?;
        let body = response.body_mut();
        let bytes: hyper::body::Bytes = hyper::body::to_bytes(body).await?;
        let json: Value = serde_json::from_slice(bytes.as_ref())?;
        match json.get("ok") {
            Some(Value::Bool(true)) => {
                let result: S = serde_json::from_value(json)?;
                Ok(result)
            }
            Some(Value::Bool(false)) => {
                let error: ErrorResponse = serde_json::from_value(json)?;
                Err(anyhow!("Request failed: {}", error.error))
            }
            _ => Err(anyhow!("unknown error: {:?}", json))
        }
    }
}
