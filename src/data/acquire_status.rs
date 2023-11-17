use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AcquireStatus {
  Ready,
  Requested,
  Completed,
  CancelRequested,
  Canceled,
}
