// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate actix_multipart_rfc7578 as actix_multipart;

use actix_multipart::client::multipart;
use awc::Client;

#[actix_rt::main]
async fn main() {
    let addr = "http://127.0.0.1:9001";
    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    let response = Client::default()
        .post(addr)
        .content_type(form.content_type())
        .send_body(multipart::Body::from(form))
        .await;

    if let Ok(_) = response {
        println!("done...");
    } else {
        eprintln!("an error occurred");
    }
}
