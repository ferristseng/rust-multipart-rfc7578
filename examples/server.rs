// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "actix")]
extern crate actix_web;

extern crate http;
#[cfg(feature = "hyper")]
extern crate hyper;

extern crate futures;

#[cfg(feature = "actix")]
use actix_web::{server, App, HttpMessage, HttpRequest, HttpResponse};

#[cfg(feature = "hyper")]
use hyper::{service::service_fn, Body, Request, Response, Server};

use futures::prelude::*;

#[cfg(feature = "hyper")]
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

#[cfg(feature = "hyper")]
fn index(req: Request<Body>) -> BoxFut {
    let res = req.into_body().concat2().map(|bod| {
        println!("{}", String::from_utf8_lossy(&bod));

        Response::new(Body::empty())
    });

    Box::new(res)
}

#[cfg(feature = "actix")]
fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().streaming(req.payload())
}
/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
///
fn main() {
    let addr = "127.0.0.1:9001";
    #[cfg(feature = "hyper")]
    {
        let server = Server::bind(&addr.parse().unwrap())
            .serve(|| service_fn(index))
            .map_err(|e| eprintln!("{}", e));

        hyper::rt::run(server);
    };
    #[cfg(feature = "actix")]
    server::new(|| App::new().resource("/", |r| r.with(index)))
        .bind(addr)
        .unwrap()
        .run();
}
