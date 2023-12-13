use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::CashReceipt;
use crate::endpoint::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ListCashReceipts {
  pub request_date: String,
  #[builder(default)]
  pub cursor: Option<u64>,
  #[builder(default)]
  pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ListCashReceiptsResponse {
  pub has_next: bool,
  pub last_cursor: u64,
  pub data: Vec<CashReceipt>,
}

impl Endpoint for ListCashReceipts {
  type Query = Self;
  type Body = ();
  type Response = ListCashReceiptsResponse;

  fn relative_path(&self) -> String {
    "/v1/cash-receipts".to_string()
  }

  fn method(&self) -> Method {
    Method::GET
  }

  fn query(&self) -> Option<&Self::Query> {
    Some(&self)
  }
}
