// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate hyper_multipart_rfc7578 as hyper_multipart;

use hyper::{Client, Request};
use hyper_multipart::client::multipart;

#[tokio::main]
async fn main() {
    let addr = "http://127.0.0.1:9001";
    let client = Client::builder().build_http();

    println!("note: this must be run in the root of the project repository");
    println!("note: run this with the example server running");
    println!("connecting to {}...", addr);

    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    let req_builder = Request::post(addr);

    let req = form.set_body::<multipart::Body>(req_builder).unwrap();

    match client.request(req).await {
        Ok(_) => {
            println!("done...");
        }
        Err(err) => {
            eprintln!("an error occurred: {:?}", err);
        }
    }
}
