use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Endpoint {
  type Query: Serialize;
  type Body: Serialize;
  type Response: DeserializeOwned;

  fn relative_path(&self) -> String;
  fn method(&self) -> reqwest::Method;
  fn query(&self) -> Option<&Self::Query> {
    None
  }
  fn body(&self) -> Option<&Self::Body> {
    None
  }
  fn idempotency_key(&self) -> Option<String> {
    None
  }
}
