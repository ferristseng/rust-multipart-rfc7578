// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate actix_multipart_rfc7578 as actix_multipart;
extern crate actix_web;
extern crate futures;

use actix_multipart::client::multipart;
use actix_web::client;
use futures::future::Future;

fn main() {
    let addr = "http://127.0.0.1:9001";
    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    actix_web::actix::run(|| {
        client::post(addr)
            .streaming(multipart::Body::from(form))
            .unwrap()
            .send()
            .map(|_| println!("done..."))
            .map_err(|_| println!("an error occurred"))
            .then(|_| { actix_web::actix::System::current().stop(); Ok(()) })
    });
}
