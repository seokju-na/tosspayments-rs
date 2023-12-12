use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::Billing;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct IssueBillingByAuthKey {
  pub auth_key: String,
  pub customer_key: String,
}

impl Endpoint for IssueBillingByAuthKey {
  type Query = ();
  type Body = Self;
  type Response = Billing;

  fn relative_path(&self) -> String {
    "/v1/billing/authorizations/issue".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
