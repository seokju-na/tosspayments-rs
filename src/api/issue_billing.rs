use crate::data::Billing;
use crate::endpoint::Endpoint;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueBilling {
  pub auth_key: String,
  pub customer_key: String,
}

impl Endpoint for IssueBilling {
  type Query = ();
  type Body = Self;
  type Response = Billing;

  fn relative_path(&self) -> String {
    "/v1/billing/authorizations/issue".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<Self::Body> {
    Some(self.clone())
  }
}
