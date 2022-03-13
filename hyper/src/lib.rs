// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578) for
//! [hyper](https://hyper.rs/).
//!
//! ## Usage
//!
//! Declare the dependency:
//!
//! ```toml
//! [dependencies]
//! hyper-multipart-rfc7578 = "0.7"
//! ```
//!
//! Import the crate:
//!
//! ```rust
//! extern crate hyper_multipart_rfc7578 as multipart;
//! ```
//!
//! ## Example:
//!
//! With a custom client:
//!
//! ```rust
//! extern crate hyper_multipart_rfc7578 as hyper_multipart;
//!
//! use hyper::{Client, Request};
//! use hyper_multipart::client::{self, multipart};
//!
//! #[tokio::main]
//! async fn main() {
//!   let client = Client::builder().build_http();
//!   let mut form = multipart::Form::default();
//!
//!   form.add_text("test", "Hello World");
//!
//!   let mut req_builder = Request::get("http://localhost/upload");
//!   let req = form.set_body::<multipart::Body>(req_builder).unwrap();
//!
//!   if let Ok(_) = client.request(req).await {
//!     println!("done...");
//!   } else {
//!     eprintln!("an error occurred");
//!   }
//! }
//! ```
//!
//! With a default client:
//!
//! ```rust
//! extern crate hyper_multipart_rfc7578 as hyper_multipart;
//!
//! use hyper::{
//!     Client, Request,
//! };
//! use hyper_multipart::client::{self, multipart};
//!
//! #[tokio::main]
//! async fn main() {
//!   let client = Client::new();
//!   let mut form = multipart::Form::default();
//!
//!   form.add_text("test", "Hello World");
//!
//!   let mut req_builder = Request::get("http://localhost/upload");
//!   let req = form.set_body_convert::<hyper::Body, multipart::Body>(req_builder)
//!       .unwrap();
//!
//!   if let Ok(_) = client.request(req).await {
//!     println!("done...");
//!   } else {
//!     eprintln!("an error occurred");
//!   }
//! }
//! ```
//!

#![allow(clippy::needless_doctest_main)]

extern crate common_multipart_rfc7578 as common_multipart;

mod body;

pub mod client {
    pub use common_multipart::client::Error;

    pub mod multipart {
        pub use crate::body::Body;
        pub use common_multipart::client::multipart::{BoundaryGenerator, Form};
    }
}
