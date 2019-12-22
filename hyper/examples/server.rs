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

use futures::Future;
use futures::TryFutureExt;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::pin::Pin;

type BoxFut = Pin<Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>>;

fn index(req: Request<Body>) -> BoxFut {
    let res = hyper::body::to_bytes(req.into_body()).map_ok(|bod| {
        println!("{}", String::from_utf8_lossy(&bod));

        Response::new(Body::empty())
    });

    Box::pin(res)
}

/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
///
fn main() {
    let addr = "127.0.0.1:9001".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(make_service_fn(|_| {
            async { Ok::<_, hyper::Error>(service_fn(index)) }
        }))
        .map_err(|e| eprintln!("{}", e));

    futures::executor::block_on(server).unwrap();
}
