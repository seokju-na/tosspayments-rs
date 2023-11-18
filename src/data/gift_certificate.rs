use serde::{Deserialize, Serialize};

use crate::data::SettlementStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftCertificate {
  pub approve_no: String,
  pub settlement_status: SettlementStatus,
}
