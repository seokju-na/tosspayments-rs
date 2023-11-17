use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum VirtualAccountType {
  #[serde(rename = "일반")]
  Normal,
  #[serde(rename = "고정")]
  Fixed,
}
