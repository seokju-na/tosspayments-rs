use serde::{Deserialize, Serialize};

use crate::data::{BankCode, SettlementStatus};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
  pub bank_code: BankCode,
  pub settlement_status: SettlementStatus,
}
