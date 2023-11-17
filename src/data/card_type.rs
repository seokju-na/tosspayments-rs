use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardType {
  #[serde(rename = "신용")]
  Credit,
  #[serde(rename = "체크")]
  Debit,
  #[serde(rename = "기프트")]
  Gift,
  #[serde(rename = "미확인")]
  Unknown,
}
