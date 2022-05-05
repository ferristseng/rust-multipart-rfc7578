// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).
//!
//! ## Usage
//!
//! Add either the Hyper implementation or the Actix implementation to
//! your Cargo.toml file:
//!
//! ### Actix:
//!
//! ```toml
//! [dependencies]
//! actix-multipart-rfc7578 = "0.10"
//! ```
//!
//! and import:
//!
//! ```rust
//! extern crate actix_multipart_rfc7578 as multipart;
//! ```
//!
//! ### Hyper:
//!
//! ```toml
//! [dependencies]
//! hyper-multipart-rfc7578 = "0.8"
//! ```
//!
//! and import:
//!
//! ```rust
//! extern crate hyper_multipart_rfc7578 as multipart;
//! ```
//!

mod boundary;
mod client_;
mod error;

pub mod client {
    pub use crate::error::Error;

    /// This module contains data structures for building a multipart/form
    /// body to send a server.
    ///
    pub mod multipart {
        pub use crate::{
            boundary::BoundaryGenerator,
            client_::{Body, Form},
        };
    }
}
