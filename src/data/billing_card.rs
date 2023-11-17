use crate::data::{CardCode, CardType, OwnerType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingCard {
  pub issuer_code: CardCode,
  pub acquirer_code: CardCode,
  pub number: String,
  pub card_type: CardType,
  pub owner_type: OwnerType,
}
