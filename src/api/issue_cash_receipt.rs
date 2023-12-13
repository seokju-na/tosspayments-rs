use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::{CashReceipt, CashReceiptType};
use crate::endpoint::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct IssueCashReceipt {
  pub amount: i32,
  pub order_id: String,
  pub order_name: String,
  pub customer_identity_number: String,
  pub r#type: CashReceiptType,
  #[builder(default)]
  pub tax_free_amount: i32,
}

impl Endpoint for IssueCashReceipt {
  type Query = ();
  type Body = Self;
  type Response = CashReceipt;

  fn relative_path(&self) -> String {
    "/v1/cash-receipts".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
