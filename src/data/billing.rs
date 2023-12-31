use serde::{Deserialize, Serialize};

use crate::data::{BillingCard, PaymentMethod};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Billing {
  pub m_id: String,
  pub customer_key: String,
  pub authenticated_at: String,
  pub method: PaymentMethod,
  pub billing_key: String,
  pub card: BillingCard,
}
