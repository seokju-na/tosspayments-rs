use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::data::{SubmallAccount, SubmallType};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Submall {
  /// 서브몰의 ID입니다. 최대 길이는 20자입니다.
  pub sub_mall_id: String,
  /// 서브몰의 타입입니다
  pub r#type: SubmallType,
  /// 서브몰의 상호명입니다. 최대 길이는 공백을 포함한 한글 30자, 영문 60자입니다. 서브몰의 `type`이 `CORPORATE`(법인)일 때만 사용됩니다.
  pub company_name: Option<String>,
  /// 서브몰의 대표자명입니다. 최대 길이는 40자입니다. 서브몰의 `type`이 `CORPORATE`(법인)일 때만 사용됩니다.
  pub representative_name: Option<String>,
  /// 서브몰의 사업자등록번호 입니다. 길이는 10자입니다. 서브몰의 `type`이 `CORPORATE`(법인)일 때만 사용됩니다.
  pub business_number: Option<String>,
  /// 서브몰에서 정산 금액을 지급받을 계좌 정보를 담은 객체입니다.
  pub account: SubmallAccount,
  /// 서브몰 이메일 주소입니다.
  pub email: Option<String>,
  /// 서브몰 연락처입니다. `-` 없이 숫자만 넣어야 합니다. 길이는 8자 이상 11자 이하여야 합니다.
  pub phone_number: Option<String>,
  /// 서브몰과 관련된 추가 정보를 key-value 쌍으로 담고 있는 객체입니다. 최대 50개의 key-value 쌍을 포함할 수 있으며 전체 크기는 4kB 이하입니다.
  pub metadata: Option<HashMap<String, String>>,
}
