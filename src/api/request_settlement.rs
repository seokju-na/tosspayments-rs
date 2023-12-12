use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::endpoint::Endpoint;

#[derive(Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RequestSettlement {
  pub payment_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestSettlementResult {
  pub result: bool,
}

impl Endpoint for RequestSettlement {
  type Query = ();
  type Body = Self;
  type Response = RequestSettlementResult;

  fn relative_path(&self) -> String {
    "/v1/settlements".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
