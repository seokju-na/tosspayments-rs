use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::Settlement;
use crate::endpoint::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListSettlementsDateType {
  #[serde(rename = "soldDate")]
  SoldDate,
  #[serde(rename = "paidOutDate")]
  PaidOutDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ListSettlements {
  pub start_date: String,
  pub end_date: String,
  #[builder(default)]
  pub date_type: Option<ListSettlementsDateType>,
  #[builder(default)]
  pub page: Option<usize>,
  #[builder(default)]
  pub size: Option<usize>,
}

impl Endpoint for ListSettlements {
  type Query = Self;
  type Body = ();
  type Response = Vec<Settlement>;

  fn relative_path(&self) -> String {
    "/v1/settlements".to_string()
  }

  fn method(&self) -> Method {
    Method::GET
  }

  fn query(&self) -> Option<&Self::Query> {
    Some(&self)
  }
}
