use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
  Ready,
  InProgress,
  WaitingForDeposit,
  Done,
  Canceled,
  PartialCanceled,
  Aborted,
  Expired,
}
