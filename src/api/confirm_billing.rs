use crate::data::Payment;
use crate::endpoint::Endpoint;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmBilling {
  pub billing_key: String,
  pub body: ConfirmBillingBody,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmBillingBody {
  pub amount: i32,
  pub customer_key: String,
  pub order_id: String,
  pub order_name: String,
  pub card_installment_plan: Option<i32>,
  pub customer_email: Option<String>,
  pub customer_name: Option<String>,
  pub tax_free_amount: Option<i32>,
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

  fn body(&self) -> Option<Self::Body> {
    Some(self.body.clone())
  }
}
