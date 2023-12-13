use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::Transaction;
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactions {
  pub start_date: String,
  pub end_date: String,
  #[builder(default)]
  pub starting_after: Option<String>,
  #[builder(default)]
  pub limit: Option<usize>,
}

impl Endpoint for ListTransactions {
  type Query = Self;
  type Body = ();
  type Response = Vec<Transaction>;

  fn relative_path(&self) -> String {
    "/v1/transactions".to_string()
  }

  fn method(&self) -> Method {
    Method::GET
  }

  fn query(&self) -> Option<&Self::Query> {
    Some(&self)
  }
}


#[cfg(test)]
mod tests {
  use httpmock::prelude::*;
  use serde_json::json;

  use crate::Client;

  use super::*;

  #[tokio::test]
  async fn list_transactions() {
    let dummy_transactions = json!([
      {
        "mId": "tvivarepublica",
        "transactionKey": "42B9BB5ECF555F940E064C5D18CBE3BE",
        "paymentKey": "tviva20231208004903pIlD7",
        "orderId": "MC4yNTE3MzkyNzQyNDMw",
        "method": "가상계좌",
        "customerKey": null,
        "useEscrow": false,
        "receiptUrl": "https://pgweb.tosspayments.com:9091/MpFlowCtrl?eventDiv1=search&eventDiv2=getCasReceiptList&trxid=tviva20231208004903pIlD7&SYSTEM=NEW",
        "status": "WAITING_FOR_DEPOSIT",
        "transactionAt": "2023-12-08T00:49:57+09:00",
        "currency": "KRW",
        "amount": 0
      },
      {
        "mId": "tvivarepublica",
        "transactionKey": "5CA936D26AA197D55B4C1BE3DA247057",
        "paymentKey": "xMljweGQBN5OWRapdA8dPgLNgN7O2b8o1zEqZKLPbmD70vk4",
        "orderId": "MC42ODEwOTg1MjU2MTI4",
        "method": "카드",
        "customerKey": null,
        "useEscrow": false,
        "receiptUrl": "https://dashboard.tosspayments.com/receipt/redirection?transactionId=tviva20231208005013dMup7&ref=PX",
        "status": "DONE",
        "transactionAt": "2023-12-08T00:50:13+09:00",
        "currency": "KRW",
        "amount": 1000
      },
      {
        "mId": "tvivarepublica2",
        "transactionKey": "5D73B312295618836EC12DD3F58FF41D",
        "paymentKey": "xMljweGQBN5OWRapdA8dq7v2OaB42b3o1zEqZKLPbmD70vk4",
        "orderId": "018c452f-cc02-e55f-cef0-a47f1885f5f4",
        "method": "카드",
        "customerKey": "018b0955-3fd3-db31-8b94-0314dd3906d4",
        "useEscrow": false,
        "receiptUrl": "https://dashboard.tosspayments.com/receipt/redirection?transactionId=tviva20231208015009eZvH9&ref=PX",
        "status": "DONE",
        "transactionAt": "2023-12-08T01:50:09+09:00",
        "currency": "KRW",
        "amount": 4900
      }
    ]);
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
      when
        .method(GET)
        .path("/v1/transactions")
        .query_param("startDate", "2023-12-08")
        .query_param("endDate", "2023-12-09")
        .query_param("limit", "3")
        .header("authorization", "Basic bXlfc2VjcmV0X2tleTo");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(dummy_transactions);
    });
    let client = Client::from_url(server.base_url(), "my_secret_key");
    let settlements = client
      .execute(&ListTransactions::builder()
        .start_date("2023-12-08".to_string())
        .end_date("2023-12-09".to_string())
        .limit(Some(3))
        .build()
      )
      .await
      .unwrap();
    mock.assert();
    assert_eq!(settlements.len(), 3);
  }
}
