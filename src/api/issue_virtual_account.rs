use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::{BankCode, CashReceiptType, Payment};
use crate::endpoint::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountCashReceipt {
  pub r#type: CashReceiptType,
  pub registration_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccountEscrowProduct {
  pub id: String,
  pub name: String,
  pub code: String,
  pub unit_price: i32,
  pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct IssueVirtualAccount {
  pub amount: i32,
  pub order_id: String,
  pub order_name: String,
  pub customer_name: String,
  pub bank: BankCode,
  #[builder(default)]
  pub account_key: Option<String>,
  #[builder(default)]
  pub valid_hours: Option<i32>,
  #[builder(default)]
  pub due_date: Option<String>,
  #[builder(default)]
  pub customer_email: Option<String>,
  #[builder(default)]
  pub customer_mobile_phone: Option<String>,
  #[builder(default)]
  pub tax_free_amount: Option<usize>,
  #[builder(default)]
  pub use_escrow: Option<bool>,
  #[builder(default)]
  pub cash_receipt: Option<VirtualAccountCashReceipt>,
  #[builder(default)]
  pub escrow_products: Option<Vec<VirtualAccountEscrowProduct>>,
}

impl Endpoint for IssueVirtualAccount {
  type Query = ();
  type Body = Self;
  type Response = Payment;

  fn relative_path(&self) -> String {
    "/v1/virtual-accounts".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
