use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BankCode {
  #[serde(rename = "39")]
  경남은행,
  #[serde(rename = "34")]
  광주은행,
  #[serde(rename = "12")]
  지역농축협,
  #[serde(rename = "32")]
  부산은행,
  #[serde(rename = "45")]
  새마을금고,
  #[serde(rename = "64")]
  산림조합,
  #[serde(rename = "88")]
  신한은행,
  #[serde(rename = "48")]
  신협,
  #[serde(rename = "27")]
  씨티은행,
  #[serde(rename = "20")]
  우리은행,
  #[serde(rename = "71")]
  우체국예금보험,
  #[serde(rename = "50")]
  저축은행중앙회,
  #[serde(rename = "37")]
  전북은행,
  #[serde(rename = "35")]
  제주은행,
  #[serde(rename = "90")]
  카카오뱅크,
  #[serde(rename = "92")]
  토스뱅크,
  #[serde(rename = "81")]
  하나은행,
  #[serde(rename = "54")]
  홍콩상하이은행,
  #[serde(rename = "03")]
  IBK기업은행,
  #[serde(rename = "06")]
  KB국민은행,
  #[serde(rename = "31")]
  DGB대구은행,
  #[serde(rename = "02")]
  KDB산업은행,
  #[serde(rename = "11")]
  NH농협은행,
  #[serde(rename = "23")]
  SC제일은행,
  #[serde(rename = "07")]
  Sh수협은행,
  #[serde(rename = "S8")]
  교보증권,
  #[serde(rename = "SE")]
  대신증권,
  #[serde(rename = "SK")]
  메리츠증권,
  #[serde(rename = "S5")]
  미래에셋증권,
  #[serde(rename = "SM")]
  부국증권,
  #[serde(rename = "S3")]
  삼성증권,
  #[serde(rename = "SN")]
  신영증권,
  #[serde(rename = "S2")]
  신한금융투자,
  #[serde(rename = "S0")]
  유안타증권,
  #[serde(rename = "SJ")]
  유진투자증권,
  #[serde(rename = "SQ")]
  카카오페이증권,
  #[serde(rename = "SB")]
  키움증권,
  #[serde(rename = "ST")]
  토스증권,
  #[serde(rename = "SR")]
  펀드온라인코리아,
  #[serde(rename = "SH")]
  하나금융투자,
  #[serde(rename = "S9")]
  하이투자증권,
  #[serde(rename = "S6")]
  한국투자증권,
  #[serde(rename = "SG")]
  한화투자증권,
  #[serde(rename = "SA")]
  현대차증권,
  #[serde(rename = "SI")]
  DB금융투자,
  #[serde(rename = "S4")]
  KB증권,
  #[serde(rename = "SP")]
  KTB투자증권,
  #[serde(rename = "SO")]
  LIG투자증권,
  #[serde(rename = "SL")]
  NH투자증권,
  #[serde(rename = "SD")]
  SK증권,
}
