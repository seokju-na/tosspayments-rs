use crate::data::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Failure {
  pub code: ErrorCode,
  pub message: String,
}
