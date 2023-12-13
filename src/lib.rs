//! `tosspayments-rs`는 [토스페이먼츠](https://www.tosspayments.com/) HTTP API를 사용하기 위한 Rust 바인딩과
//! 타입을 제공합니다.
//!
//! ## 시작하기
//!
//! [Client]을 만들어 HTTP API 사용이 가능합니다.
//!
//! API 사용에 필요한 데이터는 Rust struct로 제공됩니다. 예를들어, `payment_key`를 이용해 `Payment` 객체를 가져오는
//! API는 아래처럼 사용이 가능합니다.
//!
//! ```
//! use tosspayments::{Client, Error, api, data};
//!
//! async fn get_payment() -> Result<data::Payment, Error> {
//!   let client = Client::new("test_sk_내시크릿키");
//!   let payment = client.execute(&api::GetPayment::PaymentKey("payment_key".to_string())).await?;
//!   Ok(payment)
//! }
//! ```
//!
//! API 사용에 대한 자세한 내용은 [토스페이먼츠 공식 문서](https://docs.tosspayments.com/reference)를 참고해주세요.

#![forbid(unsafe_code)]
#![allow(clippy::large_enum_variant)]

pub use client::*;
pub use error::*;

pub mod api;
mod client;
pub mod data;
mod endpoint;
mod error;
