use serde::{Deserialize, Serialize};

use crate::data::{PaymentMethod, PaymentStatus};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
  pub m_id: String,
  pub transaction_key: String,
  pub payment_key: String,
  pub order_id: String,
  pub method: PaymentMethod,
  pub customer_key: Option<String>,
  pub use_escrow: bool,
  pub receipt_url: String,
  pub status: PaymentStatus,
  pub transaction_at: String,
  pub currency: String,
  pub amount: i32,
}
