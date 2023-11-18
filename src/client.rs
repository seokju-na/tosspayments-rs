use std::str::FromStr;

use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;

use crate::data::Failure;
use crate::endpoint::Endpoint;

static USER_AGENT: &str = concat!("Tosspayments/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct Client {
  client: reqwest::Client,
  api_base: Url,
}

impl Client {
  pub fn new(secret_key: impl Into<String>) -> Result<Self, crate::Error> {
    let auth_str = format!("{}:", secret_key.into());
    let auth = base64::engine::general_purpose::STANDARD_NO_PAD.encode(auth_str.as_bytes());
    let mut headers = HeaderMap::new();
    headers.insert("user-agent", HeaderValue::from_static(USER_AGENT));
    headers.insert(
      "accept",
      HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert(
      "authorization",
      HeaderValue::from_str(&format!("Basic {}", auth)).expect("wrong secret key"),
    );
    let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()?;
    Ok(Self {
      client,
      api_base: Url::from_str("https://api.tosspayments.com").unwrap(),
    })
  }

  pub async fn execute<E>(&self, endpoint: &E) -> Result<E::Response, crate::Error>
  where
    E: Endpoint,
  {
    let mut url = self.api_base.clone();
    url.set_path(&endpoint.relative_path());
    if let Some(query) = endpoint.query() {
      let query_str = serde_qs::to_string(&query)?;
      url.set_query(Some(&query_str));
    }
    let mut request = self.client.request(endpoint.method(), url);
    if let Some(ref body) = endpoint.body() {
      request = request.header("content-type", HeaderValue::from_static("application/json"));
      request = request.json(body);
    }
    if let Some(ref idempotency_key) = endpoint.idempotency_key() {
      request = request.header("idempotency-key", idempotency_key);
    }
    let resp = request.send().await?;
    if !resp.status().is_success() {
      let failure = resp.json::<Failure>().await?;
      return Err(crate::Error::Tosspayments {
        code: failure.code,
        message: failure.message,
      });
    }
    let result = resp.json::<E::Response>().await?;
    Ok(result)
  }
}
