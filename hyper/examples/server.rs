// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate http;
extern crate hyper;

use futures::{Future, Stream};
use hyper::{service::service_fn, Body, Request, Response, Server};

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn index(req: Request<Body>) -> BoxFut {
    let res = req.into_body().concat2().map(|bod| {
        println!("{}", String::from_utf8_lossy(&bod));

        Response::new(Body::empty())
    });

    Box::new(res)
}

/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
///
fn main() {
    let addr = "127.0.0.1:9001".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(|| service_fn(index))
        .map_err(|e| eprintln!("{}", e));

    hyper::rt::run(server);
}
