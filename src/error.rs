use crate::data::ErrorCode;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  SerdeQs(#[from] serde_qs::Error),
  #[error(transparent)]
  Tosspayments(#[from] TosspaymentsError),
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{code} ({http_status}) with message: {message:?}")]
pub struct TosspaymentsError {
  #[serde(skip_deserializing)]
  pub http_status: u16,
  pub code: ErrorCode,
  pub message: String,
}
