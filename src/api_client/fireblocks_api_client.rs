use hyper::{Client, Request, Method, Body, Error, Response};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned};

use crate::{auth::{ApiTokenProvider, BaseApiTokenProvider}, FireblocksError};
use async_trait::async_trait;


pub enum FireblocksPageMode {
    Enabled,
    Disabled
}

#[async_trait]
pub trait FireblocsApiExecutor {
    async fn issue_get_request<T: DeserializeOwned>(&self, path: &str, page_mode: FireblocksPageMode) -> Result<T, FireblocksError>;
    async fn issue_post_request<T: DeserializeOwned>(&self, path: &str, body: Option<String>, idempotency_key: Option<String>) -> Result<T, FireblocksError>;
    async fn issue_put_request<T: DeserializeOwned>(&self, path: &str, body: Option<String>) -> Result<T, FireblocksError>;
    async fn issue_delete_request<T: DeserializeOwned>(&self, path: &str) -> Result<T, FireblocksError>;
}

#[derive(Clone)]
pub struct FireblocksApiClient {
    api_token_provider: BaseApiTokenProvider,
    base_url: String,
}

impl FireblocksApiClient {
    pub fn new(provider: BaseApiTokenProvider, base_url: Option<String>) -> Self{

        let base_url = match base_url {
            Some(url) => url,
            None => "https://api.fireblocks.io".to_string()
        };

        Self { api_token_provider: provider, base_url: base_url }
    }
}

#[async_trait]
impl FireblocsApiExecutor for FireblocksApiClient {
    async fn issue_get_request<T: DeserializeOwned>(&self, path: &str, _: FireblocksPageMode) -> Result<T, FireblocksError> {
        let req = Request::builder().method(Method::GET).uri(format!("{}{}", self.base_url, path.clone()))
            .header("X-API-Key", self.api_token_provider.get_api_key())
            .header("Authorization", format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)));

        let req = req.body(Body::empty()).expect("request builder");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(req).await;
        process_fireblocks_response(response).await
    }

    async fn issue_post_request<T: DeserializeOwned>(&self, path: &str, body: Option<String>, idempotency_key: Option<String>) -> Result<T, FireblocksError> {
        let mut req = Request::builder().method(Method::POST).uri(format!("{}{}", self.base_url, path.clone()))
            .header("X-API-Key", self.api_token_provider.get_api_key())
            .header("Authorization", format!("Bearer {}", self.api_token_provider.sign_jwt(path, body.clone())))
            .header("Content-Type", "application/json");

        match idempotency_key {
            Some(idempotency_key) => {
                req = req.header("Idempotency-Key", idempotency_key);
            }
            None => {}
        }

        let body = match body {
            Some(payload) => Body::from(payload),
            None => Body::empty(),
        };

        let req = req.body(body).expect("request builder");

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(req).await;

        process_fireblocks_response(response).await
    }

    async fn issue_put_request<T: DeserializeOwned>(&self, path: &str, body: Option<String>) -> Result<T, FireblocksError> {
        let req = Request::builder().method(Method::POST).uri(format!("{}{}", self.base_url, path.clone()))
            .header("X-API-Key", self.api_token_provider.get_api_key())
            .header("Authorization", format!("Bearer {}", self.api_token_provider.sign_jwt(path, body.clone())))
            .header("Content-Type", "application/json");

        let body = match body {
            Some(payload) => Body::from(payload),
            None => Body::empty(),
        };

        let req = req.body(body).expect("request builder");

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(req).await;

        process_fireblocks_response(response).await
    }

    async fn issue_delete_request<T: DeserializeOwned>(&self, path: &str) -> Result<T, FireblocksError> {
        let req = Request::builder().method(Method::GET).uri(format!("{}{}", self.base_url, path.clone()))
            .header("X-API-Key", self.api_token_provider.get_api_key())
            .header("Authorization", format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)))
            .header("Content-Type", "application/json");

        let req = req.body(Body::empty()).expect("request builder");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(req).await;

        process_fireblocks_response(response).await
    }
}

async fn get_body(response: Response<Body>) -> Vec<u8> {
    let body = response.into_body();
    let full_body = hyper::body::to_bytes(body).await.unwrap();
    full_body.iter().cloned().collect::<Vec<u8>>()
}

async fn process_fireblocks_response<T: DeserializeOwned>(response: Result<Response<Body>, Error>) -> Result<T, FireblocksError>{
    match response {
        Ok(response) => {
            let is_success = response.status().is_success();
            let body = get_body(response).await;
            
            let result = match is_success{
                true => {
                    match serde_json::from_slice(&body[..]){
                        Ok(result) => Ok(result),
                        Err(err) => Err(FireblocksError::ResponseSerializeError(err.to_string()))
                    }
                },
                false => Err(FireblocksError::serialize_error(body)),
            };

            result
        },
        Err(err) => Err(FireblocksError::NetworkError(err.to_string()))
    }
}