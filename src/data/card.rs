use crate::data::{AcquireStatus, CardCode, CardType, InterestPayer, OwnerType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
  pub amount: i32,
  pub issuer_code: CardCode,
  pub acquirer_code: Option<CardCode>,
  pub number: String,
  pub installment_plan_months: u8,
  pub approve_no: String,
  pub use_card_point: bool,
  pub card_type: CardType,
  pub owner_type: OwnerType,
  pub acquire_status: AcquireStatus,
  pub is_interest_free: bool,
  pub interest_payer: Option<InterestPayer>,
}
