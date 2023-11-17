use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasyPayProvider {
  Tosspay,
  Naverpay,
  Samsungpay,
  Applepay,
  Lpay,
  Kakaopay,
  Pinpay,
  Payco,
  Ssg,
}
