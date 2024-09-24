// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use actix_http::body::{BodySize, MessageBody};
use bytes::Bytes;
use common_multipart::client::multipart;
use futures_core::{ready, Stream};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// Wraps a
/// [`common_multipart::client::multipart::Body`] and makes it compatible with Actix.
pub struct Body<'a>(multipart::Body<'a>);

impl<'a> From<multipart::Form<'a>> for Body<'a> {
    fn from(form: multipart::Form<'a>) -> Body<'a> {
        Body(multipart::Body::from(form))
    }
}

impl<'a> MessageBody for Body<'a> {
    type Error = common_multipart::client::Error;

    fn size(&self) -> BodySize {
        BodySize::Stream
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let Body(ref mut inner) = Pin::into_inner(self);

        match ready!(Stream::poll_next(Pin::new(inner), cx)) {
            Some(Ok(bytes)) => Poll::Ready(Some(Ok(bytes.freeze()))),
            Some(Err(err)) => Poll::Ready(Some(Err(err))),
            None => Poll::Ready(None),
        }
    }
}
