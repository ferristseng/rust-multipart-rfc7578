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
extern crate tokio_core;

mod client_;

pub mod client {
    pub mod multipart {
        pub use client_::{Body, Form, Part, BoundaryGenerator};
    }

    use hyper::client::{Client, Config, HttpConnector};
    use tokio_core::reactor::Handle;

    /// Creates a hyper client with a multipart body.
    ///
    /// ```
    /// # extern crate hyper_multipart_rfc7578;
    /// # extern crate hyper;
    /// # extern crate tokio_core;
    /// #
    /// use hyper_multipart_rfc7578::client::{self, multipart};
    /// use hyper::client::{Client, HttpConnector};
    /// use tokio_core::reactor::{Core, Handle};
    ///
    /// # fn main() {
    /// let core = Core::new().unwrap();
    /// let client: Client<HttpConnector, multipart::Body> = client::create(&core.handle());
    /// # }
    /// ```
    ///
    pub fn create(handle: &Handle) -> Client<HttpConnector, multipart::Body> {
        Config::default()
            .body::<multipart::Body>()
            .build(handle)
    }
}
