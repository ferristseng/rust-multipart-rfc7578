// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate rand;

mod client_;

pub mod client {
    pub mod multipart {
        pub use client_::{Body, Form, Part, BoundaryGenerator};
    }
}
