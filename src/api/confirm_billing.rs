use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::Payment;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmBilling {
  pub billing_key: String,
  pub body: ConfirmBillingBody,
}

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmBillingBody {
  pub amount: i32,
  pub customer_key: String,
  pub order_id: String,
  pub order_name: String,
  #[builder(default)]
  pub card_installment_plan: Option<i32>,
  #[builder(default)]
  pub customer_email: Option<String>,
  #[builder(default)]
  pub customer_name: Option<String>,
  #[builder(default)]
  pub tax_free_amount: Option<i32>,
  #[builder(default)]
  pub tax_exemption_amount: Option<i32>,
}

impl Endpoint for ConfirmBilling {
  type Query = ();
  type Body = ConfirmBillingBody;
  type Response = Payment;

  fn relative_path(&self) -> String {
    format!("/v1/billing/{}", self.billing_key)
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self.body)
  }
}
