use crate::data::SettlementStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobilePhone {
  pub customer_mobile_phone: String,
  pub settlement_status: SettlementStatus,
  pub receipt_url: String,
}
