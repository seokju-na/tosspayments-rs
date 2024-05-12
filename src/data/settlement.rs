use serde::{Deserialize, Serialize};

use crate::data::{Cancel, Card, EasyPay, Fee, GiftCertificate, MobilePhone, PaymentMethod, Transfer, VirtualAccount};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settlement {
  pub m_id: String,
  pub payment_key: String,
  pub transaction_key: String,
  pub order_id: String,
  pub currency: String,
  pub method: PaymentMethod,
  pub amount: i32,
  pub interest_fee: i32,
  pub fees: Vec<Fee>,
  pub supply_amount: i32,
  pub vat: i32,
  pub pay_out_amount: i32,
  pub approved_at: String,
  pub sold_date: String,
  pub paid_out_date: String,
  pub card: Option<Card>,
  pub easy_pay: Option<EasyPay>,
  pub gift_certificate: Option<GiftCertificate>,
  pub mobile_phone: Option<MobilePhone>,
  pub transfer: Option<Transfer>,
  pub virtual_account: Option<VirtualAccount>,
  pub cancel: Option<Cancel>,
}
