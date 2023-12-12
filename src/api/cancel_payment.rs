use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::{Account, Payment};
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CancelPayment {
  pub payment_key: String,
  pub body: CancelPaymentBody,
  #[builder(default)]
  pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CancelPaymentBody {
  pub cancel_reason: String,
  #[builder(default)]
  pub cancel_amount: Option<i32>,
  #[builder(default)]
  pub refund_receive_account: Option<Account>,
  #[builder(default)]
  pub tax_free_amount: Option<i32>,
}

impl Endpoint for CancelPayment {
  type Query = ();
  type Body = CancelPaymentBody;
  type Response = Payment;

  fn relative_path(&self) -> String {
    format!("/v1/payments/{}/cancel", self.payment_key)
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self.body)
  }

  fn idempotency_key(&self) -> Option<String> {
    self.idempotency_key.clone()
  }
}
