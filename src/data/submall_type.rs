use serde::{Deserialize, Serialize};

/// 서브몰의 타입입니다.
/// `CORPORATE`(법인), `INDIVIDUAL`(개인) 중 하나입니다.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubmallType {
  Corporate,
  Individual,
}
