// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate actix;
extern crate actix_multipart_rfc7578 as actix_multipart;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate futures01;

use actix_multipart::client::multipart;
use actix_web::client::Client;
use bytes::buf::Buf;
use futures::stream::TryStreamExt;
use futures01::future::{lazy, Future};
use futures01::Stream;

fn main() {
    let addr = "http://127.0.0.1:9001";
    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    actix::System::new("test")
        .block_on(lazy(|| {
            Client::default()
                .post(addr)
                .content_type(form.content_type())
                .send_stream(
                    multipart::Body::from(form)
                        .compat()
                        .map(|bytes| actix_web::web::Bytes::from(bytes.bytes())),
                )
                .map_err(|err| {
                    println!("an error occurred");
                    err
                })
                .and_then(|_| {
                    println!("done...");
                    actix::System::current().stop();
                    Ok(())
                })
        }))
        .unwrap();
}
