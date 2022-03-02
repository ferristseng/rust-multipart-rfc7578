// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::BytesMut;
use common_multipart::client::{multipart, Error};
use futures_core::{ready, Stream};
use http::{HeaderMap, HeaderValue};
use hyper::{self, body::HttpBody};
use std::iter::FromIterator;
use std::pin::Pin;
use std::task::{Context, Poll};

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

impl HttpBody for Body {
    type Data = BytesMut;
    type Error = Error;

    /// Implement `Payload` so `Body` can be used with a hyper client.
    ///
    #[inline]
    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let Body(inner) = Pin::into_inner(self);

        match ready!(Pin::new(inner).poll_next(cx)) {
            Some(Ok(read)) => Poll::Ready(Some(Ok(BytesMut::from_iter(read.into_iter())))),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
            None => Poll::Ready(None),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _: &mut Context,
    ) -> Poll<Result<Option<HeaderMap<HeaderValue>>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}
