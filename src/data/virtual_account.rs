use serde::{Deserialize, Serialize};

use crate::data::{Account, BankCode, RefundStatus, SettlementStatus, VirtualAccountType};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualAccount {
  pub account_type: VirtualAccountType,
  pub account_number: String,
  pub bank_code: BankCode,
  pub customer_name: String,
  pub due_date: String,
  pub refund_status: RefundStatus,
  pub expired: bool,
  pub settlement_status: SettlementStatus,
  pub refund_receive_account: Option<Account>,
}
