// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::{Bytes, IntoBuf};
use common_multipart::client::{multipart, Error};
use futures::{stream::Stream, Async, Poll};
use hyper::{self, body::Payload};
use std::io::Cursor;

pub struct Body(multipart::Body<'static>);

impl Into<hyper::Body> for Body {
    #[inline]
    fn into(self) -> hyper::Body {
        let Body(inner) = self;

        hyper::Body::wrap_stream(inner)
    }
}

impl From<multipart::Body<'static>> for Body {
    #[inline]
    fn from(body: multipart::Body<'static>) -> Self {
        Body(body)
    }
}

impl Payload for Body {
    type Data = Cursor<Bytes>;

    type Error = Error;

    /// Implement `Payload` so `Body` can be used with a hyper client.
    ///
    #[inline]
    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        let Body(inner) = self;

        match inner.poll() {
            Ok(Async::Ready(read)) => Ok(Async::Ready(read.map(IntoBuf::into_buf))),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}
