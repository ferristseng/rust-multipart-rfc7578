// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures_util::{Future, TryFutureExt};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};

fn index(req: Request<Body>) -> impl Future<Output = Result<Response<Body>, hyper::Error>> {
    println!("{:?}", req.headers());
    hyper::body::to_bytes(req.into_body()).map_ok(|bod| {
        println!("{}", String::from_utf8_lossy(&bod));

        Response::new(Body::empty())
    })
}

/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
///
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(index))
    }));

    if let Err(e) = server.await {
        eprintln!("Server Error: {}", e);
    }
}
