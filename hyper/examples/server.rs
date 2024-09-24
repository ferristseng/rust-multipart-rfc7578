// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::body::Incoming;
use hyper::server::conn::http1::Builder;
use hyper::{service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn index(req: Request<Incoming>) -> Result<Response<Empty<Bytes>>, hyper::Error> {
    println!("{:?}", req.headers());

    let data = req.into_body().collect().await?.to_bytes();
    println!("{}", String::from_utf8_lossy(&data));

    Ok(Response::new(Empty::new()))
}

/// This example runs a server that prints requests as it receives them.
/// It is useful for debugging.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _addr) = listener.accept().await?;
        tokio::spawn(async {
            let conn = Builder::new().serve_connection(TokioIo::new(socket), service_fn(index));

            match conn.await {
                Ok(_) => eprintln!("Done serving connection."),
                Err(e) => eprintln!("Server Error: {e}"),
            }
        });
    }
}
