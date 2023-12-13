//! `tosspayments-rs`는 [토스페이먼츠](https://www.tosspayments.com/) HTTP API를 사용하기 위한 Rust 바인딩과 타입을 제공합니다.
//!

#![forbid(unsafe_code)]
#![allow(clippy::large_enum_variant)]

pub use client::*;
pub use error::*;

pub mod api;
mod client;
pub mod data;
mod endpoint;
mod error;
