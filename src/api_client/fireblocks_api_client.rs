use flurl::{FlUrl, FlUrlResponse};
use serde::de::DeserializeOwned;

use async_trait::async_trait;

use super::{ApiTokenProvider, BaseApiTokenProvider, FireblocksError};

pub enum FireblocksPageMode {
    Enabled,
    Disabled,
}

#[async_trait]
pub trait FireblocsApiExecutor {
    async fn issue_get_request<T: DeserializeOwned>(
        &self,
        path: &str,
        page_mode: FireblocksPageMode,
    ) -> Result<Option<T>, FireblocksError>;
    async fn issue_post_request<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<Vec<u8>>,
        idempotency_key: Option<String>,
    ) -> Result<T, FireblocksError>;
    async fn issue_put_request<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<Vec<u8>>,
    ) -> Result<T, FireblocksError>;
    async fn issue_delete_request<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, FireblocksError>;
}

#[derive(Clone)]
pub struct FireblocksApiClient {
    api_token_provider: BaseApiTokenProvider,
    base_url: String,
}

impl FireblocksApiClient {
    pub fn new(provider: BaseApiTokenProvider, base_url: Option<String>) -> Self {
        let base_url = match base_url {
            Some(url) => url,
            None => "https://api.fireblocks.io".to_string(),
        };

        Self {
            api_token_provider: provider,
            base_url: base_url,
        }
    }
}

#[async_trait]
impl FireblocsApiExecutor for FireblocksApiClient {
    async fn issue_get_request<T: DeserializeOwned>(
        &self,
        path: &str,
        _: FireblocksPageMode,
    ) -> Result<Option<T>, FireblocksError> {
        let response = FlUrl::new(self.base_url.as_str())
            .append_path_segment(path)
            .with_header("X-API-Key", self.api_token_provider.get_api_key())
            .with_header(
                "Authorization",
                format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)),
            )
            .get()
            .await?;

        /*
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", self.base_url, path.clone()))
            .header("X-API-Key", self.api_token_provider.get_api_key())
            .header(
                "Authorization",
                format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)),
            );

        let req = req.body(Body::empty()).expect("request builder");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(req).await;
         */
        let result = process_fireblocks_response(response).await?;
        return Ok(result);
    }

    async fn issue_post_request<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<Vec<u8>>,
        idempotency_key: Option<String>,
    ) -> Result<T, FireblocksError> {
        let mut fl_url = FlUrl::new(self.base_url.as_str())
            .append_path_segment(path)
            .with_header("X-API-Key", self.api_token_provider.get_api_key())
            .with_header(
                "Authorization",
                format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)),
            )
            .with_header("Content-Type", "application/json");

        match idempotency_key {
            Some(idempotency_key) => {
                fl_url = fl_url.with_header("Idempotency-Key", idempotency_key);
            }
            None => {}
        }

        let response = fl_url.post(body).await?;

        //.post()
        //.await
        //.unwrap();

        let result = process_fireblocks_response(response).await?;
        return Ok(result.unwrap());
    }

    async fn issue_put_request<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<Vec<u8>>,
    ) -> Result<T, FireblocksError> {
        let fl_url = FlUrl::new(self.base_url.as_str())
            .append_path_segment(path)
            .with_header("X-API-Key", self.api_token_provider.get_api_key())
            .with_header(
                "Authorization",
                format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)),
            )
            .with_header("Content-Type", "application/json");

        let response = fl_url.post(body).await?;
        /*
               let req = Request::builder()
                   .method(Method::POST)
                   .uri(format!("{}{}", self.base_url, path.clone()))
                   .header("X-API-Key", self.api_token_provider.get_api_key())
                   .header(
                       "Authorization",
                       format!(
                           "Bearer {}",
                           self.api_token_provider.sign_jwt(path, body.clone())
                       ),
                   )
                   .header("Content-Type", "application/json");

               let body = match body {
                   Some(payload) => Body::from(payload),
                   None => Body::empty(),
               };

               let req = req.body(body).expect("request builder");

               let https = HttpsConnector::new();
               let client = Client::builder().build::<_, hyper::Body>(https);
               let response = client.request(req).await;
        */
        let result = process_fireblocks_response(response).await?;
        return Ok(result.unwrap());
    }

    async fn issue_delete_request<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, FireblocksError> {
        let fl_url = FlUrl::new(self.base_url.as_str())
            .append_path_segment(path)
            .with_header("X-API-Key", self.api_token_provider.get_api_key())
            .with_header(
                "Authorization",
                format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)),
            )
            .with_header("Content-Type", "application/json");

        let response = fl_url.get().await.unwrap();
        /*
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("{}{}", self.base_url, path.clone()))
            .header("X-API-Key", self.api_token_provider.get_api_key())
            .header(
                "Authorization",
                format!("Bearer {}", self.api_token_provider.sign_jwt(path, None)),
            )
            .header("Content-Type", "application/json");

        let req = req.body(Body::empty()).expect("request builder");
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let response = client.request(req).await;

         */
        let result = process_fireblocks_response(response).await?;
        return Ok(result.unwrap());
    }
}

async fn process_fireblocks_response<T: DeserializeOwned>(
    response: FlUrlResponse,
) -> Result<Option<T>, FireblocksError> {
    if response.get_status_code() == 404 {
        return Ok(None);
    }

    let body = response.receive_body().await?;

    match serde_json::from_slice(body.as_slice()) {
        Ok(result) => Ok(result),
        Err(err) => Err(FireblocksError::ResponseSerializeError(
            err.to_string(),
            String::from_utf8(body).unwrap(),
        )),
    }
}
