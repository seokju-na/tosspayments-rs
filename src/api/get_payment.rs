use reqwest::Method;

use crate::data::Payment;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub enum GetPayment {
  PaymentKey(String),
  OrderId(String),
}

impl Endpoint for GetPayment {
  type Query = ();
  type Body = ();
  type Response = Payment;

  fn relative_path(&self) -> String {
    match self {
      Self::PaymentKey(payment_key) => format!("/v1/payments/{}", payment_key),
      Self::OrderId(order_id) => format!("/v1/payments/orders/{}", order_id),
    }
  }

  fn method(&self) -> Method {
    Method::GET
  }
}

#[cfg(test)]
mod tests {
  use httpmock::prelude::*;
  use serde_json::json;

  use crate::Client;

  use super::*;

  #[tokio::test]
  async fn get_payment_by_payment_key() {
    let dummy_payment = json!({
      "mId": "tvivarepublica4",
      "lastTransactionKey": "7F8E548429D327BFE5ACEAF508E8DC36",
      "paymentKey": "Kl56WYb7w4vZnjEJeQVxn0ZyqmKWbD8PmOoBN0k12dzgRG9p",
      "orderId": "14854200076307017",
      "orderName": "www.hilti.co.kr",
      "taxExemptionAmount": 0,
      "status": "DONE",
      "requestedAt": "2023-12-01T00:05:06+09:00",
      "approvedAt": "2023-12-01T00:05:07+09:00",
      "useEscrow": false,
      "cultureExpense": false,
      "card": {
        "issuerCode": "4V",
        "acquirerCode": "21",
        "number": "40000000****109*",
        "installmentPlanMonths": 0,
        "isInterestFree": false,
        "interestPayer": null,
        "approveNo": "00000000",
        "useCardPoint": false,
        "cardType": "미확인",
        "ownerType": "미확인",
        "acquireStatus": "READY",
        "amount": 163020
      },
      "virtualAccount": null,
      "transfer": null,
      "mobilePhone": null,
      "giftCertificate": null,
      "cashReceipt": null,
      "cashReceipts": null,
      "discount": null,
      "cancels": null,
      "secret": null,
      "type": "NORMAL",
      "easyPay": null,
      "country": "KR",
      "failure": null,
      "isPartialCancelable": false,
      "receipt": {
        "url": "https://dashboard.tosspayments.com/receipt/redirection?transactionId=tviva20231201000506vM4Q2&ref=PX"
      },
      "checkout": {
        "url": "https://api.tosspayments.com/v1/payments/Kl56WYb7w4vZnjEJeQVxn0ZyqmKWbD8PmOoBN0k12dzgRG9p/checkout"
      },
      "currency": "KRW",
      "totalAmount": 163020,
      "balanceAmount": 163020,
      "suppliedAmount": 148200,
      "vat": 14820,
      "taxFreeAmount": 0,
      "method": "카드",
      "version": "2022-11-16"
    });
    let payment_key = "Kl56WYb7w4vZnjEJeQVxn0ZyqmKWbD8PmOoBN0k12dzgRG9p";
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
      when
        .method(GET)
        .path(format!("/v1/payments/{}", payment_key))
        .header("authorization", "Basic bXlfc2VjcmV0X2tleTo");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(dummy_payment);
    });
    let client = Client::from_url(server.base_url(), "my_secret_key");
    let payment = client
      .execute(&GetPayment::PaymentKey(payment_key.to_string()))
      .await
      .unwrap();
    mock.assert();
    assert_eq!(payment.payment_key, payment_key);
  }

  #[tokio::test]
  async fn get_payment_by_order_id() {
    let dummy_payment = json!({
      "mId": "tvivarepublica4",
      "lastTransactionKey": "7F8E548429D327BFE5ACEAF508E8DC36",
      "paymentKey": "Kl56WYb7w4vZnjEJeQVxn0ZyqmKWbD8PmOoBN0k12dzgRG9p",
      "orderId": "14854200076307017",
      "orderName": "www.hilti.co.kr",
      "taxExemptionAmount": 0,
      "status": "DONE",
      "requestedAt": "2023-12-01T00:05:06+09:00",
      "approvedAt": "2023-12-01T00:05:07+09:00",
      "useEscrow": false,
      "cultureExpense": false,
      "card": {
        "issuerCode": "4V",
        "acquirerCode": "21",
        "number": "40000000****109*",
        "installmentPlanMonths": 0,
        "isInterestFree": false,
        "interestPayer": null,
        "approveNo": "00000000",
        "useCardPoint": false,
        "cardType": "미확인",
        "ownerType": "미확인",
        "acquireStatus": "READY",
        "amount": 163020
      },
      "virtualAccount": null,
      "transfer": null,
      "mobilePhone": null,
      "giftCertificate": null,
      "cashReceipt": null,
      "cashReceipts": null,
      "discount": null,
      "cancels": null,
      "secret": null,
      "type": "NORMAL",
      "easyPay": null,
      "country": "KR",
      "failure": null,
      "isPartialCancelable": false,
      "receipt": {
        "url": "https://dashboard.tosspayments.com/receipt/redirection?transactionId=tviva20231201000506vM4Q2&ref=PX"
      },
      "checkout": {
        "url": "https://api.tosspayments.com/v1/payments/Kl56WYb7w4vZnjEJeQVxn0ZyqmKWbD8PmOoBN0k12dzgRG9p/checkout"
      },
      "currency": "KRW",
      "totalAmount": 163020,
      "balanceAmount": 163020,
      "suppliedAmount": 148200,
      "vat": 14820,
      "taxFreeAmount": 0,
      "method": "카드",
      "version": "2022-11-16"
    });
    let order_id = "14854200076307017";
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
      when
        .method(GET)
        .path(format!("/v1/payments/orders/{}", order_id))
        .header("authorization", "Basic bXlfc2VjcmV0X2tleTo");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(dummy_payment);
    });
    let client = Client::from_url(server.base_url(), "my_secret_key");
    let payment = client
      .execute(&GetPayment::OrderId(order_id.to_string()))
      .await
      .unwrap();
    mock.assert();
    assert_eq!(payment.order_id, order_id);
  }
}
