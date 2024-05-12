use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::data::{Submall, SubmallAccount, SubmallType};
use crate::endpoint::Endpoint;

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubmall {
  /// 서브몰의 ID입니다. 최대 길이는 20자입니다.
  pub sub_mall_id: String,
  /// 서브몰에서 정산 금액을 지급받을 계좌 정보를 담은 객체입니다.
  pub account: SubmallAccount,
  /// 서브몰의 타입입니다. `CORPORATE`(법인), `INDIVIDUAL`(개인) 중 하나의 값을 넣어주세요.
  pub r#type: SubmallType,
  /// 서브몰 이메일 주소입니다.
  pub email: String,
  /// 서브몰 연락처입니다. `-` 없이 숫자만 넣어야 합니다.
  pub phone_number: String,
  /// 서브몰의 상호명입니다. 서브몰의 `type`이 `CORPORATE`일 때 필수로 보내야 하는 파라미터입니다. 최대 길이는 공백을 포함한 한글 30자, 영문 60자입니다.
  pub company_name: Option<String>,
  /// 서브몰의 대표자명입니다. 서브몰의 `type`이 `CORPORATE`일 때 필수로 보내야 하는 파라미터입니다. 최대 길이는 공백을 포함한 한글 20자, 영문 40자입니다.
  pub representative_name: Option<String>,
  /// 서브몰의 사업자등록번호 입니다. 서브몰의 `type`이 `CORPORATE`일 때 필수로 보내야 하는 파라미터입니다. 길이는 10자입니다.
  pub business_number: Option<String>,
  /// 서브몰과 관련된 추가 정보를 key-value 쌍으로 담고 있는 객체입니다. 최대 50개의 key-value 쌍을 포함할 수 있으며 전체 크기는 4kB 이하여야 합니다. key와 value 모두 문자열 형식이어야 합니다.
  pub metadata: Option<HashMap<String, String>>,
}

impl Endpoint for CreateSubmall {
  type Query = ();
  type Body = Self;
  type Response = Submall;

  fn relative_path(&self) -> String {
    "/v1/payouts/sub-malls".to_string()
  }

  fn method(&self) -> Method {
    Method::POST
  }

  fn body(&self) -> Option<&Self::Body> {
    Some(&self)
  }
}
