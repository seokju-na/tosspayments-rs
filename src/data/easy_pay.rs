use crate::data::EasyPayProvider;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EasyPay {
  pub provider: EasyPayProvider,
  pub amount: i32,
  pub discount_amount: i32,
}
