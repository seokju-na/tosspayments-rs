use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::Payment;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmPayment {
  pub payment_key: String,
  pub order_id: String,
  pub amount: i32,
}

impl Endpoint for ConfirmPayment {
  type Query = ();
  type Body = Self;
  type Response = Payment;

  fn relative_path(&self) -> String {
    "/v1/payments/confirm".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
