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
//! actix-multipart-rfc7578 = "0.1.0"
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
//! # extern crate actix_web;
//! # extern crate futures;
//! # extern crate actix_multipart_rfc7578;
//!
//! use futures::Future;
//! use actix_multipart_rfc7578::client::{self, multipart};
//!
//! # fn main() {
//! let mut form = multipart::Form::default();
//!
//! form.add_text("test", "Hello World");
//!
//! actix_web::actix::run(|| {
//!     actix_web::client::get("http://localhost/upload")
//!         .streaming(multipart::Body::from(form))
//!         .unwrap()
//!         .send()
//!         .map(|_| println!("done..."))
//!         .map_err(|_| println!("an error occurred"))
//!         .then(|_| { actix_web::actix::System::current().stop(); Ok(()) })
//! });
//! # }
//! ```
//!

extern crate actix_web;
extern crate bytes;
extern crate common_multipart_rfc7578 as common_multipart;
extern crate futures;

mod body;
mod error;

pub mod client {
    pub use error::Error;

    pub mod multipart {
        pub use body::Body;
        pub use common_multipart::client::multipart::{BoundaryGenerator, Form};
    }
}
