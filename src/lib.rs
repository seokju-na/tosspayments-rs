#![forbid(unsafe_code)]

pub use client::*;
pub use error::*;

pub mod api;
mod client;
pub mod data;
mod endpoint;
mod error;
