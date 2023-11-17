use crate::data::Transaction;
use crate::endpoint::Endpoint;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTransactions {
  pub start_date: String,
  pub end_date: String,
  pub starting_after: Option<String>,
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

  fn query(&self) -> Option<Self::Query> {
    Some(self.clone())
  }
}
