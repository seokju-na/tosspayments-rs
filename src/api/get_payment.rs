use crate::data::Payment;
use crate::endpoint::Endpoint;
use reqwest::Method;

#[derive(Clone, Debug)]
pub enum GetPayment {
  PaymentKey { payment_key: String },
  OrderId { order_id: String },
}

impl Endpoint for GetPayment {
  type Query = ();
  type Body = ();
  type Response = Payment;

  fn relative_path(&self) -> String {
    match self {
      Self::PaymentKey { payment_key } => format!("/v1/payments/{}", payment_key),
      Self::OrderId { order_id } => format!("/v1/payments/orders/{}", order_id),
    }
  }

  fn method(&self) -> Method {
    Method::GET
  }
}
