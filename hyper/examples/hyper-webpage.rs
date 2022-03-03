// Copyright 2022 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate hyper_multipart_rfc7578 as hyper_multipart;

use futures_util::TryStreamExt;
use hyper::{Client, Request};
use hyper_tls::HttpsConnector;
use hyper_multipart::client::multipart;
use std::io;

#[tokio::main]
async fn main() {
    let website = "https://www.rust-lang.org/";
    let addr = "http://127.0.0.1:9001";

    println!("note: run this with the example server running");

    // Connect to rust-lang.org
    // The "Body" type here needs to be the default hyper "Body", not
    // the multipart Body.
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    println!("connecting to {}...", website);

    let website_res = client
        .get(website.parse().unwrap())
        .await
        .expect("expected to get response for rust-lang.org");

    // Build the multipart form passing in the website rseponse
    let mut form = multipart::Form::default();

    form.add_async_reader(
        "rust-lang",
        website_res
            .into_body()
            .map_err(|_error| io::Error::new(io::ErrorKind::Other, "hyper error"))
            .into_async_read(),
    );

    // Query the example server
    let multipart_client = Client::builder().build_http();
    let req = Request::post(addr);
    let req = form.set_body::<multipart::Body>(req).unwrap();

    println!("connecting to {}...", addr);

    match multipart_client.request(req).await {
        Ok(_) => {
            println!("done...");
        }
        Err(err) => {
            eprintln!("an error occurred: {:?}", err);
        }
    }
}
