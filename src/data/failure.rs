use serde::{Deserialize, Serialize};

use crate::data::ErrorCode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Failure {
  pub code: ErrorCode,
  pub message: String,
}
