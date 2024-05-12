use serde::{Deserialize, Serialize};

use crate::data::BankCode;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmallAccount {
  /// 은행 숫자 코드입니다. [은행 코드](bank-code)와 [증권사 코드](securities-code)를 참고하세요.
  ///
  /// [bank-code]: https://docs.tosspayments.com/reference/codes#%EC%9D%80%ED%96%89-%EC%BD%94%EB%93%9C
  /// [securities-code]: https://docs.tosspayments.com/reference/codes#%EC%A6%9D%EA%B6%8C%EC%82%AC-%EC%BD%94%EB%93%9C
  pub bank_code: BankCode,
  /// 계좌번호입니다. 최대 길이는 20자입니다.
  pub account_number: String,
  /// 지급받을 계좌의 예금주입니다. 최대 길이는 공백을 포함한 한글 30자, 영문 60자입니다.
  pub holder_name: Option<String>,
}
