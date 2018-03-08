// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate hyper;

use futures::future::Future;
use futures::stream::Stream;
use hyper::StatusCode;
use hyper::server::{Http, Request, Response, Service};

struct Debug;

impl Service for Debug {
    type Request = Request;

    type Response = Response;

    type Error = hyper::Error;

    type Future = Box<Future<Item = Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("{:?}", req);

        let res = req.body().concat2().map(|bod| {
            println!("{}", String::from_utf8_lossy(&bod));

            Response::new().with_status(StatusCode::Ok)
        });

        Box::new(res)
    }
}

/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
///
fn main() {
    let addr = "127.0.0.1:9001".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Debug)).unwrap();

    println!(
        "Listening on http://{} with 1 thread.",
        server.local_addr().unwrap()
    );

    server.run().unwrap();
}
