use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;

use crate::data::Failure;
use crate::endpoint::Endpoint;
use crate::TosspaymentsError;

static USER_AGENT: &str = concat!("Tosspayments/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct Client {
  client: reqwest::Client,
  api_base: Url,
}

impl Client {
  pub fn new(secret: impl Into<String>) -> Result<Self, crate::Error> {
    Self::from_url("https://api.tosspayments.com", secret)
  }

  pub fn from_url(url: impl Into<String>, secret: impl Into<String>) -> Result<Self, crate::Error> {
    let auth_str = format!("{}:", secret.into());
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
      api_base: Url::parse(&url.into()).expect("invalid url"),
    })
  }

  pub async fn execute<E>(&self, endpoint: &E) -> Result<E::Response, crate::Error>
  where
    E: Endpoint,
  {
    let mut url = self.api_base.clone();
    url.set_path(&endpoint.relative_path());
    if let Some(query) = endpoint.query() {
      let query_str = serde_qs::to_string(query)?;
      url.set_query(Some(&query_str));
    }
    let mut request = self.client.request(endpoint.method(), url);
    if let Some(body) = endpoint.body() {
      request = request.header("content-type", HeaderValue::from_static("application/json"));
      request = request.json(body);
    }
    if let Some(ref idempotency_key) = endpoint.idempotency_key() {
      request = request.header("idempotency-key", idempotency_key);
    }
    let resp = request.send().await?;
    let status = resp.status();
    if !status.is_success() {
      let failure = resp.json::<Failure>().await?;
      return Err(crate::Error::Tosspayments(TosspaymentsError {
        http_status: status.as_u16(),
        code: failure.code,
        message: failure.message,
      }));
    }
    let result = resp.json::<E::Response>().await?;
    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::api::GetPayment;
  use crate::data::ErrorCode;
  use crate::Error;
  use httpmock::prelude::*;
  use serde_json::json;

  #[tokio::test]
  async fn authorization() {
    let server = MockServer::start();
    let client = Client::from_url(server.base_url(), "my_secret_key").unwrap();
    let mock = server.mock(|when, then| {
      when
        .method(GET)
        .path("/v1/payments/my_payment_key")
        .header("authorization", "Basic bXlfc2VjcmV0X2tleTo");
      then
        .status(404)
        .header("content-type", "application/json")
        .json_body(json!({
          "code": "NOT_FOUND_PAYMENT",
          "message": "결제 정보를 찾을 수 없습니다"
        }));
    });
    let err = client
      .execute(&GetPayment::PaymentKey("my_payment_key".to_string()))
      .await
      .unwrap_err();
    mock.assert();
    assert!(matches!(
      err,
      Error::Tosspayments(TosspaymentsError {
        http_status: 404,
        code: ErrorCode::NotFoundPayment,
        ..
      })
    ));
  }
}
