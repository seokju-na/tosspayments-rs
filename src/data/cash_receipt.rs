use crate::data::CashReceiptType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashReceipt {
  pub r#type: CashReceiptType,
  pub receipt_key: String,
  pub issue_number: String,
  pub receipt_url: String,
  pub amount: i32,
  pub tax_free_amount: i32,
}
