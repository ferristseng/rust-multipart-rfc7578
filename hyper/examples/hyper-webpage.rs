// Copyright 2022 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use hyper_multipart_rfc7578 as hyper_multipart;

use bytes::Bytes;
use futures_util::TryStreamExt;
use http_body_util::{BodyDataStream, Empty};
use hyper::{Request, Uri};
use hyper_multipart::client::multipart;
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{Builder, Client},
    rt::TokioExecutor,
};
use std::io;

#[tokio::main]
async fn main() {
    let website = Uri::from_static("https://www.rust-lang.org/");
    let addr = Uri::from_static("http://127.0.0.1:9001");

    println!("note: run this with the example server running");

    // Connect to rust-lang.org
    // The "Body" type here needs to be the default hyper "Body", not
    // the multipart Body.
    let https = HttpsConnector::new();
    let client: Client<_, Empty<Bytes>> = Builder::new(TokioExecutor::new()).build(https);

    println!("connecting to {}...", website);

    let website_res = client
        .get(website)
        .await
        .expect("expected to get response for rust-lang.org");

    // Build the multipart form passing in the website rseponse
    let mut form = multipart::Form::default();

    form.add_async_reader(
        "rust-lang",
        BodyDataStream::new(website_res.into_body())
            .map_err(io::Error::other)
            .into_async_read(),
    );

    // Query the example server
    let multipart_client = Builder::new(TokioExecutor::new()).build_http();
    let req = Request::post(&addr);
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
