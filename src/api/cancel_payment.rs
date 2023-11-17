use crate::data::{Account, Payment};
use crate::endpoint::Endpoint;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelPayment {
  pub payment_key: String,
  pub body: CancelPaymentBody,
  pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelPaymentBody {
  pub cancel_reason: String,
  pub cancel_amount: Option<i32>,
  pub refund_receive_account: Option<Account>,
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

  fn body(&self) -> Option<Self::Body> {
    Some(self.body.clone())
  }

  fn idempotency_key(&self) -> Option<String> {
    self.idempotency_key.clone()
  }
}
