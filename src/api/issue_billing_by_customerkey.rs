use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::Billing;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct IssueBillingCustomerKeyBody {
  pub customer_key: String,
  pub card_number: String,
  pub card_expiration_year: String,
  pub card_expiration_month: String,
  pub customer_identity_number: String,
  #[builder(default)]
  pub card_password: Option<String>,
  #[builder(default)]
  pub customer_name: Option<String>,
  #[builder(default)]
  pub customer_email: Option<String>,
}

impl Endpoint for IssueBillingCustomerKeyBody {
  type Query = ();
  type Body = Self;
  type Response = Billing;

  fn relative_path(&self) -> String {
    "/v1/billing/authorizations/card".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
