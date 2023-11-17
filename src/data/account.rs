use crate::data::BankCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  pub bank_code: BankCode,
  pub account_number: String,
  pub holder_name: String,
}
