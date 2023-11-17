use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardCode {
  #[serde(rename = "3K")]
  기업BC,
  #[serde(rename = "46")]
  광주은행,
  #[serde(rename = "71")]
  롯데카드,
  #[serde(rename = "30")]
  KDB산업은행,
  #[serde(rename = "31")]
  BC,
  #[serde(rename = "51")]
  삼성카드,
  #[serde(rename = "38")]
  새마을금고,
  #[serde(rename = "41")]
  신한카드,
  #[serde(rename = "62")]
  신협,
  #[serde(rename = "36")]
  씨티카드,
  #[serde(rename = "33")]
  우리BC카드,
  #[serde(rename = "W1")]
  우리카드,
  #[serde(rename = "37")]
  우체국예금보험,
  #[serde(rename = "39")]
  저축은행중앙회,
  #[serde(rename = "35")]
  전북은행,
  #[serde(rename = "42")]
  제주은행,
  #[serde(rename = "15")]
  카카오뱅크,
  #[serde(rename = "3A")]
  케이뱅크,
  #[serde(rename = "24")]
  토스뱅크,
  #[serde(rename = "21")]
  하나카드,
  #[serde(rename = "61")]
  현대카드,
  #[serde(rename = "11")]
  KB국민카드,
  #[serde(rename = "91")]
  NH농협카드,
  #[serde(rename = "34")]
  Sh수협은행,
}
