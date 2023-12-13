use serde::{Deserialize, Serialize};

use crate::data::{
  Cancel, Card, CashReceiptOnPayment, Checkout, EasyPay, Failure, GiftCertificate, MobilePhone,
  PaymentMethod, PaymentStatus, PaymentType, Receipt, Transfer, VirtualAccount,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
  pub version: String,
  pub payment_key: String,
  pub r#type: PaymentType,
  pub order_id: String,
  pub order_name: String,
  pub m_id: String,
  pub currency: String,
  pub method: Option<PaymentMethod>,
  pub total_amount: i32,
  pub balance_amount: i32,
  pub status: PaymentStatus,
  pub requested_at: String,
  pub approved_at: Option<String>,
  pub use_escrow: Option<bool>,
  pub last_transaction_key: Option<String>,
  pub supplied_amount: i32,
  pub vat: i32,
  pub culture_expense: Option<bool>,
  pub tax_free_amount: i32,
  pub tax_exemption_amount: i32,
  pub cancels: Option<Vec<Cancel>>,
  pub is_partial_cancelable: bool,
  pub card: Option<Card>,
  pub virtual_account: Option<VirtualAccount>,
  pub secret: Option<String>,
  pub mobile_phone: Option<MobilePhone>,
  pub gift_certificate: Option<GiftCertificate>,
  pub transfer: Option<Transfer>,
  pub receipt: Option<Receipt>,
  pub checkout: Option<Checkout>,
  pub easy_pay: Option<EasyPay>,
  pub country: String,
  pub failure: Option<Failure>,
  pub cash_receipt: Option<CashReceiptOnPayment>,
}
