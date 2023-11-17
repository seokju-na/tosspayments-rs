use crate::data::ErrorCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  SerdeQs(#[from] serde_qs::Error),
  #[error("code={code:?}, message={message:?}")]
  Tosspayments { code: ErrorCode, message: String },
}
