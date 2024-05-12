# tosspayments-rs

![Crates.io](https://img.shields.io/crates/v/tosspayments-rs)

[토스페이먼츠](https://www.tosspayments.com/) HTTP API를 쉽게 사용할 수 있게 Rust 바인딩과 타입을 제공합니다.

## 문서

[Docs.rs](https://docs.rs/tosspayments-rs/latest/tosspayments/)

## 설치

`tosspayments-rs`는 [tokio](https://github.com/tokio-rs/tokio)과 호환되며 내부적으로 http 통신을 위해 [reqwest](https://github.com/seanmonstar/reqwest)를 `rustls`와 함께 사용중입니다.  

```toml
[dependencies]
tosspayments-rs = "0.2.0"
```

## 라이센스

MIT License
