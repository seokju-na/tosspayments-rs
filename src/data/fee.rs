use serde::{Deserialize, Serialize};

use crate::data::FeeType;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
  pub r#type: FeeType,
  pub fee: i32,
}
