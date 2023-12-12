use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CashReceiptTypeForRequest {
  #[serde(rename = "소득공제")]
  Consumer,
  #[serde(rename = "지출증빙")]
  Business,
  #[serde(rename = "미발행")]
  None,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashReceiptOnPayment {
  pub r#type: CashReceiptTypeForRequest,
  pub receipt_key: String,
  pub issue_number: String,
  pub receipt_url: String,
  pub amount: i32,
  pub tax_free_amount: i32,
}
