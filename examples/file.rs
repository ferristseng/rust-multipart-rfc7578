// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "actix")]
extern crate actix_web;
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
extern crate hyper_multipart_rfc7578 as hyper_multipart;

#[cfg(feature = "actix")]
use actix_web::client;
#[cfg(feature = "hyper")]
use hyper::{rt, Client, Request};

use futures::prelude::*;

use hyper_multipart::client::multipart;

fn main() {
    let addr = "http://127.0.0.1:9001";

    println!("note: this must be run in the root of the project repository");
    println!("note: run this with the example server running");
    println!("connecting to {}...", addr);

    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    #[cfg(feature = "hyper")]
    {
        let client = Client::builder().keep_alive(false).build_http();
        let mut req_builder = Request::post(addr);

        let req = form.set_body(&mut req_builder).unwrap();
        rt::run(
            client
                .request(req)
                .map(|_| println!("done..."))
                .map_err(|_| println!("an error occurred")),
        );
    };
    #[cfg(feature = "actix")]
    actix_web::actix::run(|| {
        client::post("http://127.0.0.1:9001")
            .streaming(multipart::Body::from(form))
            .unwrap()
            .send()
            .map(|_| println!("done..."))
            .map_err(|_| println!("an error occurred"))
    });
}
