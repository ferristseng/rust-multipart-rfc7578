// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use hyper_multipart_rfc7578 as hyper_multipart;

use hyper::{body::Body, Request, Uri};
use hyper_multipart::client::multipart;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Builder, Client},
    rt::TokioExecutor,
};
use std::{future::poll_fn, pin::pin};

#[tokio::main]
async fn main() {
    let addr = Uri::from_static("http://127.0.0.1:9001");
    let client = Builder::new(TokioExecutor::new()).build_http();

    println!("note: this must be run in the root of the project repository");
    println!("note: run this with the example server running");
    println!("connecting to {}...", addr);

    let mut form = multipart::Form::default();

    form.add_text("filename", file!());
    form.add_file("input", file!())
        .expect("source file path should exist");

    let req_builder = Request::post(addr);

    let req = form.set_body::<multipart::Body>(req_builder).unwrap();

    if let Err(err) = send_request(&client, req).await {
        eprintln!("an error occurred: {:?}", err);
    };
}

async fn send_request(
    client: &Client<HttpConnector, multipart::Body>,
    req: Request<multipart::Body>,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = client.request(req).await?;

    println!("receiving body");
    let mut body = pin!(res.into_body());
    while let Some(_) = poll_fn(|cx| body.as_mut().poll_frame(cx))
        .await
        .transpose()?
    {}
    println!("done...");

    Ok(())
}
