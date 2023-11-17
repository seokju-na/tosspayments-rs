use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CashReceiptType {
  #[serde(rename = "소득공제")]
  Consumer,
  #[serde(rename = "지출증빙")]
  Business,
  #[serde(rename = "미발행")]
  None,
}
