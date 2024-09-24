// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::Bytes;
use common_multipart::client::{multipart, Error};
use futures_core::{ready, Stream};
use hyper::body::Frame;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Body(multipart::Body<'static>);

impl From<multipart::Body<'static>> for Body {
    #[inline]
    fn from(body: multipart::Body<'static>) -> Self {
        Body(body)
    }
}

impl hyper::body::Body for Body {
    type Data = Bytes;
    type Error = Error;

    /// Implement `Payload` so `Body` can be used with a hyper client.
    #[inline]
    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let Body(inner) = Pin::into_inner(self);

        match ready!(Pin::new(inner).poll_next(cx)) {
            Some(Ok(read)) => Poll::Ready(Some(Ok(Frame::data(read.freeze())))),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
            None => Poll::Ready(None),
        }
    }
}
