// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578) for
//! [actix-web](https://github.com/actix/actix-web).
//!
//! ## Usage
//!
//! Declare the dependency:
//!
//! ```toml
//! [dependencies]
//! actix-multipart-rfc7578 = "0.3.0"
//! ```
//!
//! Import the crate:
//!
//! ```rust
//! extern crate actix_multipart_rfc7578 as multipart;
//! ```
//!
//! ## Example:
//!
//! ```rust
//! use actix_multipart_rfc7578::client::{self, multipart};
//! use actix_web::client::Client;
//!
//! #[actix_rt::main]
//! async fn main() {
//!   let mut form = multipart::Form::default();
//!
//!   form.add_text("test", "Hello World");
//!
//!   let response = Client::default()
//!     .get("http://localhost/upload")
//!     .content_type(form.content_type())
//!     .send_body(multipart::Body::from(form))
//!     .await;
//!
//!   if let Ok(_) = response {
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
mod error;

pub mod client {
    pub use crate::error::Error;

    pub mod multipart {
        pub use crate::body::Body;
        pub use common_multipart::client::multipart::{BoundaryGenerator, Form};
    }
}
