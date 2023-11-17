use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PaymentMethod {
  #[serde(rename = "카드")]
  Card,
  #[serde(rename = "가상계좌")]
  VirtualAccount,
  #[serde(rename = "간편결제")]
  Easypay,
  #[serde(rename = "휴대폰")]
  MobilePhone,
  #[serde(rename = "계좌이체")]
  Transfer,
  #[serde(rename = "문화상품권")]
  CultureGiftCertificate,
  #[serde(rename = "도서문화상품권")]
  BookGiftCertificate,
  #[serde(rename = "게임문화상품권")]
  GameGiftCertificate,
}
