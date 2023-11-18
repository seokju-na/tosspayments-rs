use serde::{Deserialize, Serialize};

use crate::data::EasyPayProvider;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EasyPay {
  pub provider: EasyPayProvider,
  pub amount: i32,
  pub discount_amount: i32,
}
