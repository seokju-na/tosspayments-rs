use serde::{Deserialize, Serialize};

use crate::data::Failure;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CashReceiptType {
  #[serde(rename = "소득공제")]
  Consumer,
  #[serde(rename = "지출증빙")]
  Business,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashReceiptIssueStatus {
  InProgress,
  Completed,
  Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashReceiptTransactionType {
  Confirm,
  Cancel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashReceipt {
  pub receipt_key: String,
  pub issue_number: String,
  pub issue_status: CashReceiptIssueStatus,
  pub amount: i32,
  pub tax_free_amount: i32,
  pub order_id: String,
  pub order_name: String,
  pub r#type: CashReceiptType,
  pub transaction_type: CashReceiptTransactionType,
  pub business_number: String,
  pub customer_identity_number: String,
  pub failure: Option<Failure>,
  pub requested_at: String,
  pub receipt_url: String,
}
