use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OwnerType {
  #[serde(rename = "개인")]
  Personal,
  #[serde(rename = "법인")]
  Business,
  #[serde(rename = "미확인")]
  Unknown,
}
