use crate::data::FeeType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
  pub r#type: FeeType,
  pub fee: i32,
}
