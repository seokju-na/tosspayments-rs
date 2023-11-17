use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cancel {
  pub cancel_amount: i32,
  pub cancel_reason: String,
  pub tax_free_amount: i32,
  pub tax_exemption_amount: i32,
  pub refundable_amount: i32,
  pub easy_pay_discount_amount: i32,
  pub canceled_at: String,
  pub transaction_key: String,
  pub receipt_key: Option<String>,
}
