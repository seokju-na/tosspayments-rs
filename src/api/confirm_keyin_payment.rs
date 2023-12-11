use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::data::Payment;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmKeyinPayment {
  pub amount: usize,
  pub order_id: String,
  pub order_name: String,
  pub card_number: String,
  pub card_expiration_year: String,
  pub card_expiration_month: String,
  pub customer_identity_number: String,
  #[builder(default)]
  pub card_password: Option<String>,
  #[builder(default)]
  pub card_installment_plan: Option<u8>,
  #[builder(default)]
  pub use_free_installment_plan: Option<bool>,
  #[builder(default)]
  pub tax_free_amount: Option<usize>,
  #[builder(default)]
  pub customer_email: Option<String>,
  #[builder(default)]
  pub customer_name: Option<String>,
}

impl Endpoint for ConfirmKeyinPayment {
  type Query = ();
  type Body = Self;
  type Response = Payment;

  fn relative_path(&self) -> String {
    "/v1/payments/key-in".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<Self::Body> {
    Some(self.clone())
  }
}
