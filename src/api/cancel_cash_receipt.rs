use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::CashReceipt;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CancelCashReceiptBody {
  #[builder(default)]
  pub amount: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CancelCashReceipt {
  pub receipt_key: String,
  pub body: Option<CancelCashReceiptBody>,
}

impl Endpoint for CancelCashReceipt {
  type Query = ();
  type Body = CancelCashReceiptBody;
  type Response = CashReceipt;

  fn relative_path(&self) -> String {
    format!("/v1/cash-receipts/{}/cancel", self.receipt_key)
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    match &self.body {
      Some(body) => Some(body),
      None => None,
    }
  }
}
