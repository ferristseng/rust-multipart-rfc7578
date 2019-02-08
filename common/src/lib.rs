// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578) for
//! hyper and actix.
//!
//! Currently, only the client-side is implemented.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! hyper-multipart-rfc7578 = "0.2.0-alpha2"
//! ```
//!
//! Because the name of this library is really wordy, I recommend shortening it:
//!
//! ```rust
//! extern crate hyper_multipart_rfc7578 as hyper_multipart;
//! ```
//!
//! Using this requires a hyper client compatible with the `multipart::Body`
//! data structure (see the documentation for more detailed examples):
//!
//! ```rust
//! # extern crate hyper;
//! # extern crate hyper_multipart_rfc7578 as hyper_multipart;
//!
//! use hyper::{
//!     rt::{self, Future},
//!     Client, Request,
//! };
//! use hyper_multipart::client::{self, multipart};
//!
//! # fn main() {
//! let client = Client::builder().build_http();
//! let mut form = multipart::Form::default();
//!
//! form.add_text("test", "Hello World");
//!
//! let mut req_builder = Request::get("http://localhost/upload");
//!
//! let req = form.set_body::<multipart::Body>(&mut req_builder).unwrap();
//!
//! rt::run(
//!     client
//!         .request(req)
//!         .map(|_| println!("done..."))
//!         .map_err(|_| println!("an error occurred")),
//! );
//! # }
//! ```
//!

extern crate bytes;
extern crate futures;
extern crate http;
extern crate mime;
extern crate rand;

mod client_;
mod error;

pub mod client {
    pub use error::Error;

    /// This module contains data structures for building a multipart/form
    /// body to send a server.
    ///
    pub mod multipart {
        pub use client_::{Body, BoundaryGenerator, Form};
    }
}
