// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! # Multipart RFC 7578
//!
//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).
//!
//! Currently, only the client-side is implemented.
//!
//! See the [`client`](/hyper_multipart_rfc7578/client/index.html)
//! module for examples of how to send a request with a multipart/form-data
//! body.
//!

extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate rand;
extern crate tokio_core;

mod client_;

/// # Multipart RFC 7578 - Client
///
/// This module contains code related to making requests with a
/// multipart/form body.
///
/// All data structures related to constructing a multipart/form body
/// are in the submodule [`multipart`](/hyper_multipart_rfc7578/client/index.html).
///
/// This module contains a helper method to build a hyper client that can
/// send a multipart body. [See](/hyper_multipart_rfc7578/client/fn.create.html).
///
/// # Examples
///
/// The most basic scenario would be creating a client with the provided
/// function.
///
/// ```
/// # extern crate hyper;
/// # extern crate hyper_multipart_rfc7578;
/// # extern crate tokio_core;
/// #
/// use hyper::{Method, Request};
/// use hyper::client::Client;
/// use hyper_multipart_rfc7578::client::{self, multipart};
/// use tokio_core::reactor::{Core, Handle};
///
/// # fn main() {
/// let mut core = Core::new().unwrap();
/// let client: Client<_, multipart::Body> = client::create(&core.handle());
/// let mut req = Request::new(Method::Get, "http://localhost/upload".parse().unwrap());
/// let mut form = multipart::Form::default();
///
/// form.add_text("test", "Hello World");
/// form.set_body(&mut req);
///
/// core.run(client.request(req));
/// # }
/// ```
///
/// You can also manually create a hyper client with a different connector,
/// or different configuration using hyper's `Config` object.
///
/// ```
/// # extern crate hyper;
/// # extern crate hyper_multipart_rfc7578;
/// # extern crate hyper_tls;
/// # extern crate tokio_core;
/// #
/// use hyper::{Method, Request};
/// use hyper::client::{Client, Config};
/// use hyper_multipart_rfc7578::client::multipart;
/// use hyper_tls::HttpsConnector;
/// use tokio_core::reactor::{Core, Handle};
///
/// # fn main() {
/// let mut core = Core::new().unwrap();
/// let client: Client<HttpsConnector<_>, multipart::Body> =
///     Config::default()
///         .body::<multipart::Body>()
///         .connector(HttpsConnector::new(2, &core.handle()).unwrap())
///         .keep_alive(true)
///         .build(&core.handle());
/// let mut req = Request::new(Method::Get, "http://localhost/upload".parse().unwrap());
/// let mut form = multipart::Form::default();
///
/// form.add_text("test", "Hello World");
/// form.set_body(&mut req);
///
/// core.run(client.request(req));
/// # }
/// ```
///
pub mod client {
    /// This module contains data structures for building a multipart/form
    /// body to send a server.
    ///
    pub mod multipart {
        pub use client_::{Body, Form, Part, BoundaryGenerator};
    }

    use hyper::client::{Client, Config, HttpConnector};
    use tokio_core::reactor::Handle;

    /// Creates a hyper client with a multipart body.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate hyper;
    /// # extern crate hyper_multipart_rfc7578;
    /// # extern crate tokio_core;
    /// #
    /// use hyper_multipart_rfc7578::client::{self, multipart};
    /// use hyper::client::Client;
    /// use tokio_core::reactor::{Core, Handle};
    ///
    /// # fn main() {
    /// let core = Core::new().unwrap();
    /// let client: Client<_, multipart::Body> = client::create(&core.handle());
    /// # }
    /// ```
    ///
    pub fn create(handle: &Handle) -> Client<HttpConnector, multipart::Body> {
        Config::default().body::<multipart::Body>().build(handle)
    }
}
